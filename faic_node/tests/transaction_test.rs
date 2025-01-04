use crate::common::setup;
use tracing::{info, warn, error};

use faic_node::wallet::{TransactionService, WalletService};
use faic_node::error::wallet::WalletError;
use faic_node::security::crypto::Ed25519Crypto;
use faic_node::network::p2p::faic_network;
use std::sync::Arc;

async_test_with_logging!(test_send_transaction, {
    let crypto = Arc::new(Ed25519Crypto::new());
    let network = Arc::new(FAICNetwork::new().await);
    
    info!("初始化服务");
    let wallet_service = WalletService::new(crypto.clone(), network.clone());
    let tx_service = TransactionService::new(network, crypto);

    info!("创建测试钱包");
    let sender = wallet_service.create_wallet().await.unwrap();
    let receiver = wallet_service.create_wallet().await.unwrap();

    info!("发送测试交易");
    let result = tx_service.send_transaction(
        &sender.address,
        &receiver.address,
        100000000,
        &sender.private_key
    ).await;

    assert!(result.is_ok());
    let tx_info = result.unwrap();
    info!("交易发送成功: {}", tx_info.tx_hash);
    assert!(!tx_info.tx_hash.is_empty());
});

// 新增带日志的测试
async_test_with_logging!(test_send_transaction_with_logging, {
    let crypto = Arc::new(Ed25519Crypto::new());
    let network = Arc::new(FAICNetwork::new().await);
    
    info!("初始化服务");
    let wallet_service = WalletService::new(crypto.clone(), network.clone());
    let tx_service = TransactionService::new(network, crypto);

    // 创建发送方钱包
    info!("创建发送方钱包");
    let sender = wallet_service.create_wallet().await.unwrap();
    info!("发送方地址: {}", sender.address);

    // 创建接收方钱包
    info!("创建接收方钱包");
    let receiver = wallet_service.create_wallet().await.unwrap();
    info!("接收方地址: {}", receiver.address);

    // 发送交易
    info!("开始发送交易");
    let result = tx_service.send_transaction(
        &sender.address,
        &receiver.address,
        100000000, // 1 FAIC
        &sender.private_key
    ).await;

    assert!(result.is_ok());
    let tx_info = result.unwrap();
    info!("交易发送成功，交易哈希: {}", tx_info.tx_hash);
    assert!(!tx_info.tx_hash.is_empty());
});

async_test_with_logging!(test_transaction_confirmation, {
    let crypto = Arc::new(Ed25519Crypto::new());
    let network = Arc::new(FAICNetwork::new().await);
    
    info!("初始化服务");
    let wallet_service = WalletService::new(crypto.clone(), network.clone());
    let tx_service = TransactionService::new(network, crypto);

    // 创建并发送交易
    let sender = wallet_service.create_wallet().await.unwrap();
    let receiver = wallet_service.create_wallet().await.unwrap();
    
    info!("发送测试交易");
    let tx = tx_service.send_transaction(
        &sender.address,
        &receiver.address,
        100000000,
        &sender.private_key
    ).await.unwrap();

    // 等待交易确认
    info!("等待交易确认");
    let status = tx_service.get_transaction_status(&tx.tx_hash).await.unwrap();
    
    // 检查交易状态变化
    match status {
        TransactionStatus::Pending => info!("交易待确认"),
        TransactionStatus::Confirming(n) => info!("交易确认数: {}", n),
        TransactionStatus::Confirmed => info!("交易已确认"),
        _ => warn!("意外的交易状态: {:?}", status),
    }
});