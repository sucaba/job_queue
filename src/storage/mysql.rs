use crate::entry::{QueueEntry, QueueEntryStatus};
use async_trait::async_trait;
use sqlx::{MySql, Pool};
use std::error::Error as StdError;
use uuid::Uuid;

pub struct MySqlStorage {
    pool: Pool<MySql>,
}

#[derive(sqlx::FromRow, Debug, Clone)]
struct EntryModel {}

#[async_trait]
impl super::Storage for MySqlStorage {
    async fn new(url: &str) -> Result<Self, Box<dyn StdError>> {
        let pool = Pool::<MySql>::connect(url).await?;
        match sqlx::migrate!("./migrations").run(&pool).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
        .unwrap();

        Ok(Self { pool })
    }

    async fn enqueue(&mut self, entry: QueueEntry) {
        sqlx::query("insert into job_queue (id, status, message) values (?, ?, ?)")
            .bind(entry.id())
            .bind(entry.status().to_u8())
            .bind(entry.data())
            .execute(&self.pool)
            .await
            .unwrap();
    }

    async fn dequeue(&mut self, count: usize) -> Vec<QueueEntry> {
        let mut tx = self.pool.begin().await.unwrap();

        let result = sqlx::query_as::<_, MySqlRecord>(
            "select * from job_queue where status = ?  limit ? for update",
        )
        .bind(QueueEntryStatus::Queued.to_u8())
        .bind(count as u32)
        .fetch_all(&mut tx)
        .await
        .unwrap()
        .into_iter()
        .map(|row| row.into())
        .collect();

        sqlx::query("update job_queue set status=? where status = ?  limit ?")
            .bind(QueueEntryStatus::Processing.to_u8())
            .bind(QueueEntryStatus::Queued.to_u8())
            .bind(count as u32)
            .execute(&mut tx)
            .await
            .unwrap();

        tx.commit().await.unwrap();

        result
    }
}

#[derive(sqlx::FromRow, Debug, Clone, PartialEq, Eq)]
struct MySqlRecord {
    id: Uuid,
    status: u8,
    message: String,
}

impl Into<QueueEntry> for MySqlRecord {
    fn into(self) -> QueueEntry {
        QueueEntry::new(self.id, QueueEntryStatus::from_u8(self.status), self.message)
    }
}
