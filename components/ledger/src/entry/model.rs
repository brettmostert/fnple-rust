use chrono::{self, DateTime, Utc};
use serde::{ Serialize};

#[derive(Debug, Serialize)]
pub struct Entry {
    pub id: Option<u32>,
    pub entity_id: uuid::Uuid,
    pub description: String,
    pub transactions: Vec<Transaction>,
    pub created_time: DateTime<Utc>,
    pub modified_time: DateTime<Utc>,
}

impl Entry {
    pub fn new(id: Option<u32>,entity_id: uuid::Uuid, description: String, transactions: Option<Vec<Transaction>>, created_time:DateTime<Utc>, modified_time:DateTime<Utc>) -> Self {
        if transactions == None {
            transactions = Some(vec![])
        }

        Self {
            id,
            entity_id,
            description,
            transactions: transactions.unwrap(),
            created_time,
            modified_time,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Transaction {
    #[derive(Debug, Serialize)]
    pub id: Option<u32>,
    pub entry_id: u32,
    pub entity_id: uuid::Uuid,
    pub description: String,
    pub credit_account_id: u32,
    pub debit_account_id: u32,
    pub created_time: DateTime<Utc>,
}
