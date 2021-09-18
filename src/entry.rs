use uuid::Uuid;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueueEntry {
    id: Uuid,
    data: String,
}

impl QueueEntry {
    pub fn new(id: Uuid, data: String) -> Self {
        Self { id, data }
    }

    /// Get a reference to the queue entry's id.
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a reference to the queue entry's data.
    pub fn data(&self) -> &str {
        self.data.as_str()
    }
}
