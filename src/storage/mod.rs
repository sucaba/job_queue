pub mod mysql;
use crate::entry::QueueEntry;
use async_trait::async_trait;
use std::error::Error as StdError;

#[async_trait]
pub trait Storage: Sized {
    async fn new(url: &str) -> Result<Self, Box<dyn StdError>>;

    async fn enqueue(&mut self, entry: QueueEntry);

    async fn dequeue(&mut self, count: usize) -> Vec<QueueEntry>;
}
