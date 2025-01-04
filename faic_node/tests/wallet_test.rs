use crate::common::setup;
use tracing::{info, warn, error};

use faic_node::wallet::{WalletService, WalletInfo, Balance};
use faic_node::error::wallet::WalletError;
use faic_node::security::crypto::Ed25519Crypto;
use faic_node::network::p2p::faic_network;
use std::sync::Arc;

async_test_with_logging!(test_create_wallet, {
    let crypto = Arc::new(Ed25519Crypto::new());
    let network = Arc::new(FAICNetwork::new().await);
    info!("初始化钱包服务");
    let wallet_service = WalletService::new(crypto, network);

    let result = wallet_service.create_wallet().await;
    info!("钱包创建结果: {:?}", result);
    assert!(result.is_ok());

    let wallet = result.unwrap();
    info!("钱包地址: {}", wallet.address);
    assert!(!wallet.address.is_empty());
    assert!(!wallet.mnemonic.is_empty());
    assert!(!wallet.private_key.is_empty());
    assert!(!wallet.segwit_address.is_empty());
});

async_test_with_logging!(test_import_wallet, {
    let crypto = Arc::new(Ed25519Crypto::new());
    let network = Arc::new(FAICNetwork::new().await);
    info!("初始化钱包服务");
    let wallet_service = WalletService::new(crypto, network);

    // 创建钱包获取助记词
    info!("创建测试钱包");
    let wallet = wallet_service.create_wallet().await.unwrap();
    let mnemonic = wallet.mnemonic.clone();
    info!("原始钱包地址: {}", wallet.address);

    // 测试导入
    info!("开始导入钱包");
    let imported = wallet_service.import_wallet(mnemonic).await;
    assert!(imported.is_ok());
    
    let imported_wallet = imported.unwrap();
    info!("导入钱包地址: {}", imported_wallet.address);
    assert_eq!(wallet.address, imported_wallet.address);
    assert_eq!(wallet.segwit_address, imported_wallet.segwit_address);
    info!("钱包导入成功，地址匹配");
});

async_test_with_logging!(test_get_balance, {
    let crypto = Arc::new(Ed25519Crypto::new());
    let network = Arc::new(FAICNetwork::new().await);
    info!("初始化钱包服务");
    let wallet_service = WalletService::new(crypto, network);

    info!("创建测试钱包");
    let wallet = wallet_service.create_wallet().await.unwrap();
    info!("钱包地址: {}", wallet.address);

    info!("查询钱包余额");
    let balance = wallet_service.get_balance(&wallet.address).await;
    assert!(balance.is_ok());
    
    let balance = balance.unwrap();
    info!("钱包余额: {}", balance.total);
    assert_eq!(balance.total.format_decimals(), "0.00000000");
});