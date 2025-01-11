// src/api/handlers/wallet.rs
use axum::{
    extract::{Path, Json},
    http::StatusCode,
};
use utoipa::path::{Parameter, Response};

#[utoipa::path(
    post,
    path = "/wallet/create",
    tag = "wallet",
    responses(
        (status = 200, description = "钱包创建成功", body = WalletCreateResponse),
        (status = 500, description = "服务器错误")
    )
)]
pub async fn create_wallet() -> Result<Json<WalletCreateResponse>, StatusCode> {
    // 实现钱包创建逻辑
    #[derive(Debug)]
    pub struct WalletCreateResponse {
        pub address: String,
        pub mnemonic: String,
    }
}

#[utoipa::path(
    get,
    path = "/wallet/balance/{address}",
    tag = "wallet",
    params(
        ("address" = String, Path, description = "钱包地址")
    ),
    responses(
        (status = 200, description = "查询成功", body = BalanceResponse),
        (status = 404, description = "地址不存在"),
        (status = 500, description = "服务器错误")
    )
)]
pub async fn get_balance(
    Path(address): Path<String>
) -> Result<Json<BalanceResponse>, StatusCode> {
    // 实现余额查询逻辑
    #[derive(Debug)]
    pub struct BalanceResponse {
        pub balance: String,
    }
}