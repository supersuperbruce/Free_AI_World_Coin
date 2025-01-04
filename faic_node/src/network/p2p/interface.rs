use async_trait::async_trait;
use crate::network::p2p::error::P2PResult;

#[async_trait]
pub trait P2PNetworkInterface: Send + Sync {
    /// 发现节点
    async fn discover_nodes(&self) -> P2PResult<Vec<String>>;
    
    /// 广播消息
    async fn broadcast(&self, message: Vec<u8>) -> P2PResult<()>;
    
    /// 发送消息到指定节点
    async fn send_to(&self, peer_id: &str, message: Vec<u8>) -> P2PResult<()>;
    
    /// 获取连接节点列表
    async fn get_peers(&self) -> P2PResult<Vec<String>>;
}