use std::sync::Mutex;

use crate::account::model::Account;

use super::repository::{FetchAllError, InsertError, Repository};

pub struct InMemoryRepository {
    error: bool,
    accounts: Mutex<Vec<Account>>,
}

impl InMemoryRepository {
    pub fn new() -> Self {
        let accounts: Mutex<Vec<Account>> = Mutex::new(vec![]);
        Self {
            error: false,
            accounts,
        }
    }
}

impl Repository for InMemoryRepository {
    fn insert(&self, new_account: Account) -> Result<Account, InsertError> {
        if self.error {
            return Err(InsertError::Unknown);
        }

        let mut lock = match self.accounts.lock() {
            Ok(lock) => lock,
            _ => return Err(InsertError::Unknown),
        };

        if lock
            .iter()
            .any(|acc| acc.entity_id == new_account.entity_id)
        {
            return Err(InsertError::Conflict);
        }
        let mut next_id = 1;
        if lock.len() != 0 {
            next_id = lock.iter().map(|acc| acc.id.unwrap()).max().unwrap() + 1;
        }

        let account = Account::new(
            Some(next_id),
            new_account.entity_id,
            new_account.description,
            new_account.created_time,
            new_account.modified_time,
        );
        lock.push(account.clone());
        Ok(account)
    }

    fn fetch_all(&self) -> Result<Vec<Account>, super::repository::FetchAllError> {
        if self.error {
            return Err(FetchAllError::Unknown);
        }

        let mut lock = match self.accounts.lock() {
            Ok(lock) => lock,
            _ => return Err(FetchAllError::Unknown),
        };

        let accounts = lock.to_vec();
        Ok(accounts)
    }

    fn fetch_by_id(&self, id: u32) -> Result<Account, super::repository::FetchOneError> {
        todo!()
    }

    fn delete_by_id(&self, id: u32) -> Result<(), super::repository::DeleteError> {
        todo!()
    }
}
