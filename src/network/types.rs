use crate::types::Amount;
use libp2p::{Multiaddr, PeerId};
use serde::{Deserialize, Serialize};

/// 节点信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NodeInfo {
    /// 节点的 PeerId
    #[serde(with = "crate::network::config::serde_peer_id")]
    pub peer_id: PeerId,
    /// 节点的地址列表
    pub addresses: Vec<Multiaddr>,
    /// 节点是否在线
    pub is_online: bool,
}

/// 消息类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MessageType {
    /// 请求
    Request,
    /// 响应
    Response,
    /// 心跳
    Heartbeat,
    /// 数据
    Data,
}

/// 请求类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Request {
    /// 查询余额
    GetBalance {
        /// 要查询的地址
        address: String, // 暂时使用 String 类型，后续可以根据需要修改为更具体的地址类型
    },
    /// 发送交易
    SendTransaction {
        /// 交易内容 (暂时使用 String 类型，后续需要定义具体的交易结构)
        transaction: String,
    },
    /// 获取节点信息
    GetNodeInfo,
}

/// 响应类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Response {
    /// 查询余额的响应
    GetBalanceResponse {
        /// 余额 
        balance: Amount,
    },
    /// 发送交易的响应
    SendTransactionResponse {
        /// 交易哈希 (暂时使用 String 类型，后续需要定义具体的哈希类型)
        tx_hash: String,
    },
    /// 获取节点信息的响应
    GetNodeInfoResponse {
        /// 节点信息
        node_info: NodeInfo,
    },
    /// 错误响应
    Error {
        /// 错误信息
        message: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Amount;
    use num_bigint::BigUint;

    #[test]
    fn test_node_info_serialization() {
        let peer_id = PeerId::random();
        let addresses = vec!["/ip4/127.0.0.1/tcp/8080".parse().unwrap()];
        let node_info = NodeInfo {
            peer_id,
            addresses,
            is_online: true,
        };

        let serialized = serde_json::to_string(&node_info).unwrap();
        let deserialized: NodeInfo = serde_json::from_str(&serialized).unwrap();

        assert_eq!(node_info.peer_id, deserialized.peer_id);
        assert_eq!(node_info.addresses, deserialized.addresses);
        assert_eq!(node_info.is_online, deserialized.is_online);
    }

    #[test]
    fn test_get_balance_request_serialization() {
        let request = Request::GetBalance {
            address: "some_address".to_string(),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: Request = serde_json::from_str(&serialized).unwrap();

        match deserialized {
            Request::GetBalance { address } => assert_eq!(address, "some_address"),
            _ => panic!("Unexpected request type"),
        }
    }

    #[test]
    fn test_get_balance_response_serialization() {
        let response = Response::GetBalanceResponse {
            balance: Amount::from_biguint(BigUint::from(100u32)).unwrap(),
        };

        let serialized = serde_json::to_string(&response).unwrap();
        let deserialized: Response = serde_json::from_str(&serialized).unwrap();

        match deserialized {
            Response::GetBalanceResponse { balance } => {
                assert_eq!(balance.value(), &BigUint::from(100u32))
            }
            _ => panic!("Unexpected response type"),
        }
    }

    #[test]
    fn test_message_type_serialization() {
        let message_types = vec![
            MessageType::Request,
            MessageType::Response,
            MessageType::Heartbeat,
            MessageType::Data,
        ];

        for message_type in message_types {
            let serialized = serde_json::to_string(&message_type).unwrap();
            let deserialized: MessageType = serde_json::from_str(&serialized).unwrap();
            assert_eq!(message_type, deserialized);
        }
    }

    #[test]
    fn test_request_serialization() {
        let requests = vec![
            Request::GetBalance {
                address: "some_address".to_string(),
            },
            Request::SendTransaction {
                transaction: "some_transaction".to_string(),
            },
            Request::GetNodeInfo,
        ];

        for request in requests {
            let serialized = serde_json::to_string(&request).unwrap();
            let deserialized: Request = serde_json::from_str(&serialized).unwrap();
            assert_eq!(request, deserialized); // 需要为 Request 实现 PartialEq
        }
    }

    #[test]
    fn test_response_serialization() {
        let responses = vec![
            Response::GetBalanceResponse {
                balance: Amount::from_biguint(BigUint::from(100u32)).unwrap(),
            },
            Response::SendTransactionResponse {
                tx_hash: "some_tx_hash".to_string(),
            },
            Response::GetNodeInfoResponse {
                node_info: NodeInfo {
                    peer_id: PeerId::random(),
                    addresses: vec!["/ip4/127.0.0.1/tcp/8080".parse().unwrap()],
                    is_online: true,
                },
            },
            Response::Error {
                message: "some_error_message".to_string(),
            },
        ];

        for response in responses {
            let serialized = serde_json::to_string(&response).unwrap();
            let deserialized: Response = serde_json::from_str(&serialized).unwrap();
            assert_eq!(response, deserialized); // 需要为 Response 实现 PartialEq
        }
    }

    #[test]
    fn test_invalid_deserialization() {
        let invalid_json = "\"invalid_json_string\"";
        let result: Result<NodeInfo, _> = serde_json::from_str(invalid_json);
        assert!(result.is_err());
    }    
}