use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

use job_queue::{storage::mysql::MySqlStorage, JobQueue, QueueEntry};
use mysql_rent::Rent;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
struct ImportFile {
    file_path: PathBuf,
}

impl ImportFile {
    pub fn new(file_path: PathBuf) -> Self {
        Self { file_path }
    }

    /// Get a reference to the import file's file path.
    pub fn file_path(&self) -> &PathBuf {
        &self.file_path
    }
}

#[tokio::test]
async fn should_create_and_use_queue() {
    let mysql = Rent::new().await.unwrap();

    let url = mysql.mysql_url();

    println!("url = {}", url);

    let mut jobs = JobQueue::<MySqlStorage>::new(&url).await.unwrap();

    let job = ImportFile::new("foo.csv".into());
    jobs.enqueue(QueueEntry::new(
        Uuid::new_v4(),
        serde_json::to_string(&job).unwrap(),
    ))
    .await;

    let pending = jobs.dequeue(100).await;

    assert_eq!(
        pending
            .into_iter()
            .map(|e| serde_json::from_str::<ImportFile>(e.data()).unwrap())
            .collect::<Vec<_>>(),
        vec![job]
    );
}
