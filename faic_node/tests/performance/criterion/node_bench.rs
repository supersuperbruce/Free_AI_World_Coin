use crate::common::setup;
use tracing::{info, warn, error};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use faic_node::network::p2p::FAICNetwork;
use std::sync::Arc;

pub fn node_benchmark(c: &mut Criterion) {
    // 初始化日志
    setup();
    info!("开始节点性能测试");

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        info!("创建网络实例");
        let network = Arc::new(FAICNetwork::new().await);

        c.bench_function("node discovery", |b| {
            info!("开始节点发现基准测试");
            b.iter(|| {
                let rt = tokio::runtime::Runtime::new().unwrap();
                rt.block_on(async {
                    info!("执行节点发现");
                    match network.discover_nodes().await {
                        Ok(_) => info!("节点发现成功"),
                        Err(e) => error!("节点发现失败: {}", e),
                    }
                });
            });
            info!("节点发现基准测试完成");
        });
    });

    info!("节点性能测试完成");
}

criterion_group!(benches, node_benchmark);
criterion_main!(benches);