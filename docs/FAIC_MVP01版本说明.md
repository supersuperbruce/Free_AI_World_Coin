# MVP001版本要求：

核心目标：实现FAIC钱包的创建、助记词导入、转账、查询余额等功能。

开发要求：
1、节点程序使用rust开发，要求网络协议采用p2p框架。
2、先开发节点程序，使用openapi规范API接口，通过curl命令行工具对各个功能测试验证，完善API文档后，再开发客户端。
3、代码中要有详细的注释，包括函数、结构体、枚举等。
4、要在代码中加入恰当的打印，方便调试。
5、客户端使用flutter开发。

# MVP001 开发计划

## 1. 必要的核心层实现

### 1.1 接口定义 (Interfaces)

```rust
/// P2P 网络核心接口
pub trait P2PNetworkInterface {
   // 节点发现与管理
   async fn discover_Node_list(&self) -> Result<Vec<NodeInfo>>;
   async fn connect_Node(&self, Node: NodeInfo) -> Result<()>;
   async fn disconnect_Node(&self, Node_id: NodeId) -> Result<()>;
   
   // 消息传输
   async fn broadcast_message(&self, message: NetworkMessage) -> Result<()>;
   async fn send_message(&self, Node_id: NodeId, message: NetworkMessage) -> Result<()>;
   
   // 状态同步
   async fn sync_state(&self) -> Result<()>;
   async fn get_Node_state(&self, Node_id: NodeId) -> Result<NodeState>;
}

/// P2P 网络实现
pub struct P2PNetwork {
   // 节点标识
   node_id: NodeId,
   // 节点类型
   node_type: NodeType,
   // 连接管理器
   connection_manager: ConnectionManager,
   // 消息处理器
   message_handler: MessageHandler,
   // 状态管理器
   state_manager: StateManager,
   // 计算资源管理
   compute_manager: ComputeManager,      
   // 验证管理
   validation_manager: ValidationManager, 
   // 奖励管理
   reward_manager: RewardManager,        
   // 安全管理(ZK-STARK和FHE)
   security_manager: SecurityManager,     
}

/// 消息类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkMessage {
   // 钱包相关
   CreateWallet(CreateWalletRequest),
   ImportWallet(ImportWalletRequest),
   GetBalance(GetBalanceRequest),
   Transfer(TransferRequest),
   
   // 节点状态
   NodeState(NodeStateMessage),
   // 节点发现
   DiscoverNodes(DiscoverNodesRequest),
   NodeList(Vec<NodeInfo>),
   
   // 响应消息
   Response(ResponseMessage),

   // 计算相关
   SubmitCompute(ComputeRequest),
   ComputeResult(ComputeResponse),
   
   // 验证相关
   ValidationRequest(ValidationRequest),
   ValidationResult(ValidationResponse),
   
   // 奖励相关
   RewardClaim(RewardClaimRequest),
   RewardDistribution(RewardDistributionMessage),
}
```

### 1.2 最小化可插拔组件

```rust
// 1. 网络组件 (Network Components)
pub struct BasicNetwork {
   Node_list: Vec<NodeInfo>,
   connection_manager: ConnectionManager,
}
// 2. 共识组件 (Consensus Components)
pub struct SimpleConsensus {
   validators: Vec<ValidatorInfo>,
}
// 3. 安全组件 (Security Components)
pub struct BasicCrypto {
   key_manager: KeyManager,
}
// 4. 验证组件 (Validation Components)
pub struct ValidationManager {
   fast_validator: FastValidator,
   block_validator: BlockValidator,
   final_validator: FinalValidator,
   zk_stark: ZkStarkValidator,
}
// 5. 计算组件 (Compute Components)
pub struct ComputeManager {
   model_manager: ModelManager,
   task_scheduler: TaskScheduler,
   resource_monitor: ResourceMonitor,
   vllm_integration: VLLMIntegration,
}
```

### 1.3 必要的服务实现

```rust
// 1. 钱包服务
pub struct WalletService {
   crypto: Arc<BasicCrypto>,
   network: Arc<BasicNetwork>,
}
// 2. 交易服务
pub struct TransactionService {
   network: Arc<BasicNetwork>,
   consensus: Arc<SimpleConsensus>,
}

// 3. 奖励服务
pub struct RewardService {
    reward_manager: Arc<RewardManager>,
    validation_manager: Arc<ValidationManager>,
    consensus: Arc<SimpleConsensus>,
}

// 4. 计算服务
pub struct ComputeService {
   compute_manager: Arc<ComputeManager>,
   validation_manager: Arc<ValidationManager>,
   network: Arc<BasicNetwork>,
}
```
## 2.错误处理和日志记录的标准接口
### 2.1 错误类型定义
```rust
/// 标准错误类型枚举
#[derive(Debug, Clone)]
pub enum FAICError {
   // 网络错误
   NetworkError(NetworkErrorKind),
   // 计算错误
   ComputeError(ComputeErrorKind),
   // 验证错误
   ValidationError(ValidationErrorKind),
   // 安全错误
   SecurityError(SecurityErrorKind),
   // 资源错误
   ResourceError(ResourceErrorKind),
   // 状态错误
   StateError(StateErrorKind),
   // 用户错误
   UserError(UserErrorKind),
}

/// 网络错误类型
#[derive(Debug, Clone)]
pub enum NetworkErrorKind {
   ConnectionFailed,
   Timeout(Duration),
   NodeNotFound(NodeId),
   MessageTooLarge(usize),
   InvalidProtocol,
}

/// 计算错误类型
#[derive(Debug, Clone)]
pub enum ComputeErrorKind {
   ResourceExhausted,
   ModelNotFound(String),
   TaskFailed(String),
   InvalidInput,
   VLLMError(String),
}

/// 验证错误类型
#[derive(Debug, Clone)]
pub enum ValidationErrorKind {
   InvalidSignature,
   InvalidBlock,
   InvalidTransaction,
   ConsensusError,
   StakeInsufficient,
}
```

### 2.2 日志接口定义
```rust
/// 日志级别
#[derive(Debug, Clone, Copy)]
pub enum LogLevel {
   Error,
   Warn,
   Info,
   Debug,
   Trace,
}

/// 日志上下文
#[derive(Debug, Clone)]
pub struct LogContext {
   timestamp: DateTime<Utc>,
   node_id: NodeId,
   node_type: NodeType,
   component: String,
   trace_id: String,
   user_id: Option<String>,
}

/// 日志记录接口
pub trait LoggerInterface: Send + Sync {
   /// 记录日志
   fn log(&self, level: LogLevel, context: LogContext, message: String);
   
   /// 记录错误
   fn error(&self, context: LogContext, error: FAICError);
   
   /// 记录指标
   fn metric(&self, context: LogContext, name: String, value: f64);
   
   /// 记录事件
   fn event(&self, context: LogContext, event_type: String, data: serde_json::Value);
}
```

### 2.3 错误处理接口
```rust
/// 错误处理接口
pub trait ErrorHandler: Send + Sync {
   /// 处理错误
   async fn handle_error(&self, error: FAICError, context: LogContext) -> Result<(), FAICError>;
   
   /// 重试策略
   async fn retry<F, T>(&self, f: F, max_retries: u32) -> Result<T, FAICError>
   where
      F: Fn() -> Future<Output = Result<T, FAICError>>;
   
   /// 错误恢复
   async fn recover(&self, error: FAICError) -> Result<(), FAICError>;
   
   /// 错误通知
   async fn notify(&self, error: FAICError, severity: LogLevel);
}

/// 错误监控接口
pub trait ErrorMonitor: Send + Sync {
   /// 监控错误率
   async fn monitor_error_rate(&self, window: Duration) -> Result<f64, FAICError>;
   
   /// 监控资源状态
   async fn monitor_resources(&self) -> Result<ResourceStatus, FAICError>;
   
   /// 监控节点健康状态
   async fn monitor_node_health(&self) -> Result<NodeHealth, FAICError>;
}
```
## 3. 节点程序开发阶段

### Phase 1: 基础设施搭建 (1周)
- 搭建项目框架
- 实现核心接口定义
- 设置基本的错误处理
- 配置日志系统

### Phase 2: 钱包核心功能 (2周)
1. 钱包创建
   - 生成私钥
   - 生成助记词
   - 创建钱包地址

2. 助记词导入
   - 助记词验证
   - 私钥恢复
   - 地址重建

### Phase 3: 网络功能 (2周)
1. 基本网络连接
   - 节点发现
   - 连接管理
   - 心跳检测

2. 交易广播
   - 交易序列化
   - 广播机制
   - 确认机制

### Phase 4: 账户功能 (1周)
1. 余额查询
   - 账户状态同步
   - 余额更新机制

2. 转账功能
   - 交易构建
   - 签名验证
   - 交易提交

### Phase 5: 测试与优化 (1周)
1. 单元测试
   - 接口测试
   - 功能测试
   - 边界测试

2. 集成测试
   - 网络测试
   - 性能测试
   - 压力测试