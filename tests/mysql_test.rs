use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

use job_queue::{storage::mysql::MySqlStorage, JobQueue, QueueEntry, QueueEntryStatus};
use mysql_rent::Rent;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
struct ImportFile {
    file_path: PathBuf,
}

impl ImportFile {
    pub fn new(file_path: PathBuf) -> Self {
        Self { file_path }
    }
}

#[tokio::test]
async fn should_create_and_use_queue() {
    let mysql = Rent::new().await.unwrap();

    let url = mysql.mysql_url();

    println!("url = {}", url);

    let mut jobs = JobQueue::<MySqlStorage>::new(&url).await.unwrap();

    let planned: Vec<ImportFile> = (0..100)
        .map(|i| ImportFile::new(format!("foo{}.csv", i).into()))
        .collect();

    for job in &planned {
        jobs.enqueue(QueueEntry::new(
            Uuid::new_v4(),
            QueueEntryStatus::Queued,
            serde_json::to_string(&job).unwrap(),
        ))
        .await;
    }

    let pending = jobs.dequeue(10).await;

    assert_eq!(
        &pending
            .into_iter()
            .map(|e| serde_json::from_str::<ImportFile>(e.data()).unwrap())
            .collect::<Vec<_>>(),
        &planned[0..10]
    );
}
