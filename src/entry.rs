use uuid::Uuid;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum QueueStatus {
    Queued = 0,
    Processing = 1,
    Done = 2,
    Failed = 3,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueueEntry {
    id: Uuid,
    status: QueueStatus,
    data: String,
}

impl QueueEntry {
    pub fn new(id: Uuid, status: QueueStatus, data: String) -> Self {
        Self { id, status, data }
    }

    /// Get a reference to the queue entry's id.
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a reference to the queue entry's status.
    pub fn status(&self) -> QueueStatus {
        self.status
    }

    /// Get a reference to the queue entry's data.
    pub fn data(&self) -> &str {
        self.data.as_str()
    }
}
