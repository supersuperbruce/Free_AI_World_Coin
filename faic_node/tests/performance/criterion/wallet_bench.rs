use crate::common::setup;
use tracing::{info, warn, error};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use faic_node::wallet::WalletService;
use faic_node::security::crypto::Ed25519Crypto;
use faic_node::network::p2p::FAICNetwork;
use std::sync::Arc;

pub fn wallet_benchmark(c: &mut Criterion) {
    // 初始化日志
    setup();
    info!("开始钱包性能测试");

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let crypto = Arc::new(Ed25519Crypto::new());
        let network = Arc::new(FAICNetwork::new().await);
        info!("初始化钱包服务");
        let wallet_service = WalletService::new(crypto, network);

        c.bench_function("create wallet", |b| {
            info!("开始钱包创建基准测试");
            b.iter(|| {
                let rt = tokio::runtime::Runtime::new().unwrap();
                rt.block_on(async {
                    let result = wallet_service.create_wallet().await;
                    match result {
                        Ok(wallet) => info!("成功创建钱包，地址: {}", wallet.address),
                        Err(e) => error!("钱包创建失败: {}", e),
                    }
                });
            });
            info!("钱包创建基准测试完成");
        });
    });

    info!("钱包性能测试完成");
}

criterion_group!(benches, wallet_benchmark);
criterion_main!(benches);