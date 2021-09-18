use async_trait::async_trait;
use sqlx::{MySql, Pool};
// use mysql_async::{prelude::*};
use crate::entry::QueueEntry;
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
        sqlx::query("insert into job_queue (id, message) values (?, ?)")
            .bind(entry.id())
            .bind(entry.data())
            .execute(&self.pool)
            .await
            .unwrap();
    }

    async fn dequeue(&mut self, count: usize) -> Vec<QueueEntry> {
        sqlx::query_as::<_, MySqlRecord>(r#"SELECT id, message from job_queue limit ?"#)
            .bind(count as u32)
            .fetch_all(&self.pool)
            .await
            .unwrap()
            .into_iter()
            .map(|row| QueueEntry::new(row.id, row.message))
            .collect()
    }
}

#[derive(sqlx::FromRow, Debug, Clone, PartialEq, Eq)]
struct MySqlRecord {
    id: Uuid,
    message: String,
}
