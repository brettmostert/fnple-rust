use axum::{http::StatusCode, response::IntoResponse, Json};
use chrono::{DateTime, Utc};
use serde::Deserialize;

use super::model::Account;

#[derive(Debug, Deserialize)]
pub struct CreateAccount {
    pub entity_id: uuid::Uuid,
    pub description: String,
}

#[allow(clippy::unused_async)]
pub async fn create_account(Json(payload): Json<CreateAccount>) -> impl IntoResponse {
    let now: DateTime<Utc> = Utc::now();
    let account = Account::new(None, payload.entity_id, payload.description, now, now);

    (StatusCode::CREATED, Json(account))
}
