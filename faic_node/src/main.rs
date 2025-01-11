use axum::{Router, routing::serve};
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    api::{docs::ApiDoc, routes::wallet_routes},
    network::p2p::FAICNetwork,
    security::crypto::Ed25519Crypto,
    wallet::{WalletService, TransactionService},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new("info"))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // 初始化核心服务
    let crypto = Arc::new(Ed25519Crypto::new());
    let network = Arc::new(FAICNetwork::new().await?);
    let wallet_service = Arc::new(WalletService::new(crypto.clone(), network.clone()));
    let tx_service = Arc::new(TransactionService::new(network.clone(), crypto.clone()));

    // 启动P2P网络
    network.start().await?;

    // 初始化API文档
    let api_doc = ApiDoc::openapi();
    
    // 创建路由
    let app = Router::new()
        .merge(wallet_routes())
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api_doc))
        .with_state(wallet_service)
        .with_state(tx_service);
    
    // 启动HTTP服务器
    let addr = "[::]:3000".parse()?;
    tracing::info!("API文档地址: http://localhost:3000/swagger-ui");
    
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}