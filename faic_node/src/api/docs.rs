use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::wallet::create_wallet,
        handlers::wallet::import_wallet,
        handlers::wallet::get_balance,
        handlers::transaction::send_transaction,
    ),
    components(
        schemas(
            models::WalletCreateResponse,
            models::WalletImportRequest,
            models::BalanceResponse,
            models::TransactionRequest,
            models::TransactionResponse
        )
    ),
    tags(
        (name = "wallet", description = "钱包管理接口"),
        (name = "transaction", description = "交易相关接口")
    )
)]
pub struct ApiDoc;