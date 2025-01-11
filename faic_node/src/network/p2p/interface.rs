use async_trait::async_trait;
use crate::error::network::NetworkError;

#[async_trait]
pub trait P2PNetworkInterface {
    async fn discover_node_list(&self) -> Result<Vec<String>, NetworkError>;
    async fn broadcast_message(&self, message: Vec<u8>) -> Result<(), NetworkError>;
}