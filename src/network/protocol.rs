use crate::network::{types::{Request, Response}, error::Error};
use libp2p::{
    core::{
        upgrade::{read_length_prefixed, write_length_prefixed},
        ProtocolName,
    },
    futures::{AsyncRead, AsyncWrite, AsyncWriteExt, StreamExt},
    request_response::{
        ProtocolSupport, RequestResponse, RequestResponseCodec, RequestResponseConfig,
        RequestResponseEvent,
    },
};
use std::io;
use std::iter;
use tokio::time::timeout;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct FaicProtocol();
#[derive(Clone)]
pub struct FaicCodec();

impl ProtocolName for FaicProtocol {
    fn protocol_name(&self) -> &[u8] {
        "/faic/1".as_bytes()
    }
}

#[async_trait::async_trait]
impl RequestResponseCodec for FaicCodec {
    type Protocol = FaicProtocol;
    type Request = Request;
    type Response = Response;

    async fn read_request<T>(
        &mut self,
        _: &Self::Protocol,
        io: &mut T,
    ) -> io::Result<Self::Request>
    where
        T: AsyncRead + Unpin + Send,
    {
        // 读取请求数据
        let data = read_length_prefixed(io, 1_024_000).await?; // 设置一个最大长度限制，例如 1MB
        if data.is_empty() {
            return Err(io::ErrorKind::UnexpectedEof.into());
        }
        // 反序列化请求
        let request: Request = serde_json::from_slice(&data).map_err(|e| {
            io::Error::new(io::ErrorKind::InvalidData, format!("Deserialize error: {}", e))
        })?;
        Ok(request)
    }

    async fn read_response<T>(
        &mut self,
        _: &Self::Protocol,
        io: &mut T,
    ) -> io::Result<Self::Response>
    where
        T: AsyncRead + Unpin + Send,
    {
        // 读取响应数据
        let data = read_length_prefixed(io, 1_024_000).await?; // 设置一个最大长度限制，例如 1MB
        if data.is_empty() {
            return Err(io::ErrorKind::UnexpectedEof.into());
        }
        // 反序列化响应
        let response: Response = serde_json::from_slice(&data).map_err(|e| {
            io::Error::new(io::ErrorKind::InvalidData, format!("Deserialize error: {}", e))
        })?;
        Ok(response)
    }

    async fn write_request<T>(
        &mut self,
        _: &Self::Protocol,
        io: &mut T,
        req: Self::Request,
    ) -> io::Result<()>
    where
        T: AsyncWrite + Unpin + Send,
    {
        // 序列化请求
        let data = serde_json::to_vec(&req).map_err(|e| {
            io::Error::new(io::ErrorKind::InvalidData, format!("Serialize error: {}", e))
        })?;
        // 写入请求数据
        write_length_prefixed(io, data).await?;
        io.close().await?;
        Ok(())
    }

    async fn write_response<T>(
        &mut self,
        _: &Self::Protocol,
        io: &mut T,
        res: Self::Response,
    ) -> io::Result<()>
    where
        T: AsyncWrite + Unpin + Send,
    {
        // 序列化响应
        let data = serde_json::to_vec(&res).map_err(|e| {
            io::Error::new(io::ErrorKind::InvalidData, format!("Serialize error: {}", e))
        })?;
        // 写入响应数据
        write_length_prefixed(io, data).await?;
        io.close().await?;
        Ok(())
    }
}

pub fn create_faic_network_behaviour() -> RequestResponse<FaicCodec> {
    // 创建 RequestResponseConfig
    let config = RequestResponseConfig::default();

    // 使用 FaicCodec 创建 RequestResponse 行为
    RequestResponse::new(
        FaicCodec(),
        iter::once((FaicProtocol(), ProtocolSupport::Full)),
        config,
    )
}

pub async fn handle_request(request: Request) -> Result<Response, Error> {
    match request {
        Request::GetBalance { address } => {
            // 在这里实现查询余额的逻辑
            // ...
            // 假设 balance 是查询到的余额
            let balance = crate::types::Amount::from_str("100")?; // 示例余额
            Ok(Response::GetBalanceResponse { balance })
        }
        Request::SendTransaction { transaction } => {
            // 在这里实现发送交易的逻辑
            // ...
            // 假设 tx_hash 是交易哈希
            let tx_hash = "some_tx_hash".to_string(); // 示例交易哈希
            Ok(Response::SendTransactionResponse { tx_hash })
        }
        Request::GetNodeInfo => {
            // 在这里实现获取节点信息的逻辑
            // ...
            // 假设 node_info 是节点信息
            let node_info = crate::network::types::NodeInfo {
                peer_id: libp2p::PeerId::random(), // 示例 PeerId
                addresses: vec!["/ip4/127.0.0.1/tcp/8080".parse().unwrap()], // 示例地址
                is_online: true, // 示例在线状态
            };
            Ok(Response::GetNodeInfoResponse { node_info })
        }
    }
}

pub async fn start_listening(transport: libp2p::core::transport::Boxed<(libp2p::PeerId, libp2p::core::muxing::StreamMuxerBox)>) -> Result<(), Error> {
    // 创建 NetworkBehaviour
    let mut behaviour = create_faic_network_behaviour();

    // 让 behaviour 监听所有接口的随机端口
    behaviour.listen_on("/ip4/0.0.0.0/tcp/0".parse().unwrap()).unwrap();

    // 启动一个异步任务来处理事件
    tokio::spawn(async move {
        loop {
            tokio::select! {
                event = behaviour.select_next_some() => match event {
                    RequestResponseEvent::Message { peer, message } => {
                        match message {
                            libp2p::request_response::RequestResponseMessage::Request { request, channel, .. } => {
                                println!("Received request from {:?}: {:?}", peer, request);
                                // 处理请求并发送响应
                                let response = match handle_request(request).await {
                                    Ok(response) => response,
                                    Err(e) => {
                                        eprintln!("Error handling request: {}", e);
                                        Response::Error { message: e.to_string() }
                                    }
                                };
                                if let Err(e) = behaviour.send_response(channel, response) {
                                    eprintln!("Failed to send response: {:?}", e);
                                }
                            }
                            libp2p::request_response::RequestResponseMessage::Response { response, .. } => {
                                println!("Received response from {:?}: {:?}", peer, response);
                            }
                        }
                    }
                    RequestResponseEvent::OutboundFailure { peer, request_id, error } => {
                        eprintln!("Outbound failure to {:?} with request {:?}: {:?}", peer, request_id, error);
                    }
                    RequestResponseEvent::InboundFailure { peer, request_id, error } => {
                        eprintln!("Inbound failure from {:?} with request {:?}: {:?}", peer, request_id, error);
                    }
                    RequestResponseEvent::ResponseSent { peer, request_id } => {
                        println!("Response sent to {:?} for request {:?}", peer, request_id);
                    }
                }
            }
        }
    });

    Ok(())
}

