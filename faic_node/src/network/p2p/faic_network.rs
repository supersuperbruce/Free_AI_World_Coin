const TOPIC_NAME: &str = "/faic/1.0.0";

use libp2p::{
    kad::{
        store::MemoryStore, Kademlia, KademliaConfig, KademliaEvent,
        record::store::MemoryStore as KademliaMemoryStore,
        QueryResult,
    },
    gossipsub::{
        Gossipsub, GossipsubConfig, GossipsubEvent, 
        MessageAuthenticity, IdentTopic
    },
    swarm::{NetworkBehaviour, Swarm, SwarmEvent},
    development_transport,
    StreamProtocol,
    PeerId,
};
use crate::network::p2p::interface::P2PNetworkInterface;
use async_trait::async_trait;
use std::sync::Arc;
use tracing::info;
use crate::error::network::NetworkError;
use tokio::sync::{mpsc, RwLock}; // Using tokio's async RwLock
use futures::StreamExt; // For .next() on Swarm

// 定义复合行为结构，包含 Kademlia 和 Gossipsub 行为
#[derive(NetworkBehaviour)]
#[behaviour(out_event = "FAICNetworkEvent")]
pub struct FAICBehaviour {
    pub kad: Kademlia<MemoryStore>,
    pub gossipsub: Gossipsub,
}

// 定义网络事件枚举
pub enum FAICNetworkEvent {
    Kad(KademliaEvent),
    Gossipsub(GossipsubEvent),
}

// 定义主要的网络结构
pub struct FAICNetwork {
    swarm: Swarm<FAICBehaviour>,
    peers: Arc<RwLock<Vec<PeerId>>>,
    message_tx: mpsc::Sender<Vec<u8>>,
    message_rx: mpsc::Receiver<Vec<u8>>,
    topic: IdentTopic,
}


impl FAICNetwork {
    pub async fn new() -> Result<Self, NetworkError> {
        info!("初始化 P2P 网络");

        // 生成节点密钥对
        let local_key = identity::Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(local_key.public());
        
        // 初始化 Kademlia 用于节点发现
        let store = MemoryStore::new(local_peer_id);
        let kad_config = KademliaConfig::default();
        let kad = Kademlia::new(local_peer_id, store, kad_config);

        // 初始化 Gossipsub 用于消息传播
        let gossipsub_config = GossipsubConfig::default();
        let gossipsub = Gossipsub::new(
            MessageAuthenticity::Signed(local_key.clone()),
            gossipsub_config
        ).map_err(|e| NetworkError::ProtocolError(e.to_string()))?;

        // 创建网络行为实例
        let behaviour = FAICBehaviour {
            kad,
            gossipsub,
        };

        // 创建并订阅主题
        let topic = IdentTopic::new(TOPIC_NAME);
        gossipsub.subscribe(&topic)?;

        // let transport = libp2p::tcp::TcpTransport::new(libp2p::tcp::Config::default())
        //     .upgrade()
        //     .authenticate(libp2p::noise::NoiseConfig::xx(local_key.clone()).into_authenticated())
        //     .multiplex(libp2p::yamux::YamuxConfig::default())
        //     .boxed();

        // 创建传输层
        let transport = development_transport(local_key).await?;

        // 创建 Swarm 用于管理网络行为
        let swarm = Swarm::new(
            transport,
            behaviour,
            local_peer_id,
            libp2p::swarm::Config::with_tokio_executor()
        );

        let (tx, rx) = mpsc::channel(100);

        Ok(Self {
            swarm,
            peers: Arc::new(RwLock::new(Vec::new())),
            message_tx: tx,
            message_rx: rx,
            topic,
        })
    }

    /// 启动网络服务
    pub async fn start(&mut self) -> Result<(), NetworkError> {
        info!("启动 P2P 网络服务");

        loop {
            tokio::select! {
                // 处理网络事件
                event = self.swarm.next() => match event {
                    Some(SwarmEvent::Behaviour(FAICNetworkEvent::Kad(kad_event))) => {
                        self.handle_kad_event(kad_event).await?;
                    }
                    Some(SwarmEvent::Behaviour(FAICNetworkEvent::Gossipsub(gossip_event))) => {
                        self.handle_gossip_event(gossip_event).await?;
                    }
                    _ => {}
                },
                // 处理接收到的消息
                Some(message) = self.message_rx.recv() => {
                    self.broadcast_message(message).await?;
                }
            }
        }
    }

    /// 处理 Kademlia 事件
    async fn handle_kad_event(&mut self, event: KademliaEvent) -> Result<(), NetworkError> {
        match event {
            KademliaEvent::OutboundQueryCompleted { result, .. } => {
                match result {
                    QueryResult::GetClosestPeers(Ok(closest_peers)) => {
                        // 更新已发现的对等节点列表
                        let mut peer_list = self.peers.write().await;
                        for peer_id in closest_peers.peers {
                            if !peer_list.iter().any(|p| p == &peer_id) {
                                info!("发现新节点: {:?}", peer_id);
                                peer_list.push(peer_id);
                            }
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        Ok(())
    }

    /// 处理 Gossipsub 事件
    async fn handle_gossip_event(&mut self, event: GossipsubEvent) -> Result<(), NetworkError> {
        match event {
            GossipsubEvent::Message { message, .. } => {
                info!("收到消息: {:?}", message);
                // 这里可以添加处理接收到的消息的逻辑
            }
            _ => {}
        }
        Ok(())
    }
}

#[async_trait]
impl P2PNetworkInterface for FAICNetwork {
    //获取已发现节点列表
    async fn discover_node_list(&self) -> Result<Vec<String>, NetworkError> {
        let peers = self.peers.read().await;
        Ok(peers.iter().map(|p| p.to_string()).collect())
    }

    //广播消息
    async fn broadcast_message(&self, message: Vec<u8>) -> Result<(), NetworkError> {
        self.swarm.behaviour_mut().gossipsub.publish(
            self.topic.clone(),
            message
        ).map_err(|e| NetworkError::BroadcastError(e.to_string()))?;
        Ok(())
    }

    // ... 其他接口实现
}