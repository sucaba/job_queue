use crate::entry::QueueEntry;
use crate::storage::Storage;
use std::error::Error as StdError;

pub struct JobQueue<S: Storage> {
    storage: S,
}

impl<S: Storage> JobQueue<S> {
    pub async fn new(url: &str) -> Result<Self, Box<dyn StdError>> {
        S::new(url).await.map(|storage| Self { storage })
    }

    pub async fn enqueue(&mut self, entry: QueueEntry) {
        self.storage.enqueue(entry).await
    }

    pub async fn dequeue(&mut self, count: usize) -> Vec<QueueEntry> {
        self.storage.dequeue(count).await
    }
}

#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use std::collections::VecDeque;
    use uuid::Uuid;

    use crate::entry::QueueStatus;

    use super::*;

    struct TestStorage {
        entries: VecDeque<QueueEntry>,
    }

    #[async_trait]
    impl Storage for TestStorage {
        async fn new(_url: &str) -> Result<Self, Box<dyn StdError>> {
            Ok(Self {
                entries: VecDeque::new(),
            })
        }

        async fn enqueue(&mut self, entry: QueueEntry) {
            self.entries.push_back(entry)
        }

        async fn dequeue(&mut self, count: usize) -> Vec<QueueEntry> {
            self.entries.drain(0..count).collect()
        }
    }

    impl Default for TestStorage {
        fn default() -> Self {
            Self {
                entries: VecDeque::new(),
            }
        }
    }

    type Sut = JobQueue<TestStorage>;

    #[tokio::test]
    async fn should_be_creatable() {
        let _ = Sut::new("any-url").await.unwrap();
    }

    #[tokio::test]
    async fn should_store_and_read() {
        let mut sut = Sut::new("any-url").await.unwrap();
        let entry = QueueEntry::new(Uuid::new_v4(), QueueStatus::Queued, "red".into());
        sut.enqueue(entry.clone()).await;
        let d = sut.dequeue(1).await.pop();
        assert_eq!(d, Some(entry));
    }
}
