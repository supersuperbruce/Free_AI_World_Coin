use std::sync::Once;
use tracing::{info, warn, error};
use tokio;

static INIT: Once = Once::new();

pub fn setup() {
    INIT.call_once(|| {
        tracing_subscriber::fmt()
            .with_test_writer()
            .with_level(true)
            .with_file(true)
            .with_line_number(true)
            .init();
    });
}

// 测试辅助宏
#[macro_export]
macro_rules! test_with_logging {
    ($name:ident, $body:expr) => {
        #[test]
        fn $name() {
            crate::common::setup();
            info!(test_name = stringify!($name), "开始测试");
            $body
            info!(test_name = stringify!($name), "测试完成");
        }
    };
}

#[macro_export]
macro_rules! async_test_with_logging {
    ($name:ident, $body:expr) => {
        #[tokio::test]
        async fn $name() {
            crate::common::setup();
            info!(test_name = stringify!($name), "开始异步测试");
            $body.await;
            info!(test_name = stringify!($name), "异步测试完成");
        }
    };
}