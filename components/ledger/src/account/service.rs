use std::sync::Arc;

use axum::extract::Extension;
use axum::{http::StatusCode, response::IntoResponse, Json};
use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::account::data::repository::Repository;

use super::{data::in_memory_repository::InMemoryRepository, model::Account};

#[derive(Debug, Deserialize)]
pub struct CreateAccount {
    pub entity_id: uuid::Uuid,
    pub description: String,
}

#[allow(clippy::unused_async)]
pub async fn create_account(
    Extension(repo): Extension<Arc<InMemoryRepository>>,
    Json(payload): Json<CreateAccount>,
) -> impl IntoResponse {
    let now: DateTime<Utc> = Utc::now();
    let account = Account::new(None, payload.entity_id, payload.description, now, now);
    match repo.insert(account.clone()) {
        Ok(new_account) => (StatusCode::CREATED, Json(new_account)),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(account)),
    }
}

#[allow(clippy::unused_async)]
pub async fn get_accounts(
    Extension(repo): Extension<Arc<InMemoryRepository>>,
) -> impl IntoResponse {
    match repo.fetch_all() {
        Ok(accounts) => (StatusCode::CREATED, Json(accounts)),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(vec![])),
    }
}
