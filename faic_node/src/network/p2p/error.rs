use thiserror::Error;

#[derive(Error, Debug)]
pub enum P2PError {
    #[error("网络连接失败: {0}")]
    ConnectionError(String),

    #[error("节点发现失败: {0}")]
    DiscoveryError(String),

    #[error("消息广播失败: {0}")]
    BroadcastError(String),

    #[error("节点离线")]
    NodeOffline,

    #[error("超时错误")]
    Timeout,
}

pub type P2PResult<T> = Result<T, P2PError>;