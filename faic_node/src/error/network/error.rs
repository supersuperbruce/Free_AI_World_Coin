use thiserror::Error;
use std::time::Duration;

#[derive(Error, Debug)]
pub enum NetworkError {
    #[error("网络连接失败: {0}")]
    ConnectionError(String),

    #[error("节点发现失败: {0}")]
    DiscoveryError(String),

    #[error("消息广播失败: {0}")]
    BroadcastError(String),

    #[error("节点离线")]
    NodeOffline,

    #[error("超时错误: {0:?}")]
    Timeout(Duration),

    #[error("节点未找到: {0}")]
    NodeNotFound(String),

    #[error("消息过大: {0} bytes")]
    MessageTooLarge(usize),

    #[error("协议错误: {0}")]
    ProtocolError(String),

    #[error("状态同步失败: {0}")]
    StateSyncError(String),
}

pub type P2PResult<T> = Result<T, P2PError>;