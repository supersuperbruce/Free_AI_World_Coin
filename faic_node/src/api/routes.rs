// src/api/routes.rs
use axum::{
    Router,
    routing::{post, get},
};
use crate::api::handlers;

pub fn wallet_routes() -> Router {
    Router::new()
        .route("/wallet/create", post(handlers::wallet::create_wallet))
        .route("/wallet/import", post(handlers::wallet::import_wallet))
        .route("/wallet/balance/:address", get(handlers::wallet::get_balance))
        .route("/transaction/send", post(handlers::transaction::send_transaction))
}