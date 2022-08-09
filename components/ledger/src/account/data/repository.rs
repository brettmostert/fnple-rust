use crate::account::model::Account;

pub enum InsertError {
    Conflict,
    Unknown,
}

pub enum FetchAllError {
    Unknown,
}

pub enum FetchOneError {
    NotFound,
    Unknown,
}

pub enum DeleteError {
    NotFound,
    Unknown,
}

pub trait Repository: Send + Sync {
    fn insert(&self, account: Account) -> Result<Account, InsertError>;

    fn fetch_all(&self) -> Result<Vec<Account>, FetchAllError>;

    fn fetch_by_id(&self, id: u32) -> Result<Account, FetchOneError>;

    fn delete_by_id(&self, id: u32) -> Result<(), DeleteError>;
}
