#![warn(clippy::all, clippy::pedantic)]
#![forbid(unsafe_code)]
use crate::account::service::create_account;
use account::{data::in_memory_repository::InMemoryRepository, service::get_accounts};
use axum::{
    routing::{get},
    Extension, Router,
};

use std::{net::SocketAddr, sync::Arc};
mod account;

#[tokio::main]
async fn main() {
    let repo = Arc::new(InMemoryRepository::new());
    let app = Router::new()
        .route("/", get(root))
        .route("/accounts", get(get_accounts).post(create_account))
        .layer(Extension(repo));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    // tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[allow(clippy::unused_async)]
async fn root() -> &'static str {
    "Hello, World!"
}
