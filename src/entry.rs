use uuid::Uuid;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum QueueEntryStatus {
    Queued = 0,
    Processing = 1,
    Done = 2,
    Failed = 3,
}

impl QueueEntryStatus {
    pub(crate) fn from_u8(value: u8) -> Self {
        use QueueEntryStatus::*;
        match value {
            0 => Queued,
            1 => Processing,
            2 => Done,
            _ => Failed,
        }
    }

    pub(crate) fn to_u8(self) -> u8 {
        match self {
            QueueEntryStatus::Queued => 0,
            QueueEntryStatus::Processing => 1,
            QueueEntryStatus::Done => 2,
            QueueEntryStatus::Failed => 3,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueueEntry {
    id: Uuid,
    status: QueueEntryStatus,
    data: String,
}

impl QueueEntry {
    pub fn new(id: Uuid, status: QueueEntryStatus, data: String) -> Self {
        Self { id, status, data }
    }

    /// Get a reference to the queue entry's id.
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a reference to the queue entry's status.
    pub fn status(&self) -> QueueEntryStatus {
        self.status
    }

    /// Get a reference to the queue entry's data.
    pub fn data(&self) -> &str {
        self.data.as_str()
    }
}
