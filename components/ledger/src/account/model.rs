use chrono::{self, DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct Account {
    pub id: Option<u32>,
    pub entity_id: uuid::Uuid,
    pub description: String,
    pub created_time: DateTime<Utc>,
    pub modified_time: DateTime<Utc>,
}

impl Account {
    pub fn new(
        id: Option<u32>,
        entity_id: uuid::Uuid,
        description: String,
        created_time: DateTime<Utc>,
        modified_time: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            entity_id,
            description,
            created_time,
            modified_time,
        }
    }
}
