
enum WalletStatus {
  uninitialized, // 未初始化
  creating, // 创建中
  ready, // 就绪
  locked, // 锁定
  error // 错误
}

class WalletModel {
  final String address; // 钱包地址
  final String privateKey; // 私钥
  final String? mnemonic; // 助记词
  final double balance; // FAIC余额
  final Map<String, double> tokenBalances; // 其他代币余额
  final WalletStatus status; // 钱包状态
  final String? password; // 钱包密码(加密存储)
  final List<Transaction> transactions; // 交易历史
  final DateTime createdAt; // 创建时间
  final DateTime? lastActive; // 最后活动时间

  WalletModel({
    required this.address,
    required this.privateKey,
    this.mnemonic,
    this.balance = 0.0,
    this.tokenBalances = const {},
    this.status = WalletStatus.uninitialized,
    this.password,
    this.transactions = const [],
    DateTime? createdAt,
    this.lastActive,
  }) : createdAt = createdAt ?? DateTime.now();

  // 从JSON构造
  factory WalletModel.fromJson(Map<String, dynamic> json) {
    return WalletModel(
      address: json['address'] as String,
      privateKey: json['privateKey'] as String,
      mnemonic: json['mnemonic'] as String?,
      balance: (json['balance'] as num?)?.toDouble() ?? 0.0,
      tokenBalances: Map<String, double>.from(json['tokenBalances'] ?? {}),
      status: WalletStatus.values[json['status'] as int? ?? 0],
      password: json['password'] as String?,
      transactions: (json['transactions'] as List<dynamic>?)
              ?.map((e) => Transaction.fromJson(e as Map<String, dynamic>))
              .toList() ??
          [],
      createdAt: DateTime.parse(json['createdAt'] as String),
      lastActive: json['lastActive'] != null
          ? DateTime.parse(json['lastActive'] as String)
          : null,
    );
  }

  // 转换为JSON
  Map<String, dynamic> toJson() {
    return {
      'address': address,
      'privateKey': privateKey,
      'mnemonic': mnemonic,
      'balance': balance,
      'tokenBalances': tokenBalances,
      'status': status.index,
      'password': password,
      'transactions': transactions.map((e) => e.toJson()).toList(),
      'createdAt': createdAt.toIso8601String(),
      'lastActive': lastActive?.toIso8601String(),
    };
  }

  // 创建钱包副本
  WalletModel copyWith({
    String? address,
    String? privateKey,
    String? mnemonic,
    double? balance,
    Map<String, double>? tokenBalances,
    WalletStatus? status,
    String? password,
    List<Transaction>? transactions,
    DateTime? createdAt,
    DateTime? lastActive,
  }) {
    return WalletModel(
      address: address ?? this.address,
      privateKey: privateKey ?? this.privateKey,
      mnemonic: mnemonic ?? this.mnemonic,
      balance: balance ?? this.balance,
      tokenBalances: tokenBalances ?? this.tokenBalances,
      status: status ?? this.status,
      password: password ?? this.password,
      transactions: transactions ?? this.transactions,
      createdAt: createdAt ?? this.createdAt,
      lastActive: lastActive ?? this.lastActive,
    );
  }
}

// 交易模型
class Transaction {
  final String hash; // 交易哈希
  final String from; // 发送方地址
  final String to; // 接收方地址
  final double amount; // 交易金额
  final String? tokenAddress; // 代币地址(如果是代币交易)
  final DateTime timestamp; // 交易时间
  final TransactionStatus status; // 交易状态

  Transaction({
    required this.hash,
    required this.from,
    required this.to,
    required this.amount,
    this.tokenAddress,
    required this.timestamp,
    this.status = TransactionStatus.pending,
  });

  factory Transaction.fromJson(Map<String, dynamic> json) {
    return Transaction(
      hash: json['hash'] as String,
      from: json['from'] as String,
      to: json['to'] as String,
      amount: (json['amount'] as num).toDouble(),
      tokenAddress: json['tokenAddress'] as String?,
      timestamp: DateTime.parse(json['timestamp'] as String),
      status: TransactionStatus.values[json['status'] as int],
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'hash': hash,
      'from': from,
      'to': to,
      'amount': amount,
      'tokenAddress': tokenAddress,
      'timestamp': timestamp.toIso8601String(),
      'status': status.index,
    };
  }
}

enum TransactionStatus {
  pending, // 待处理
  confirmed, // 已确认
  failed, // 失败
  cancelled // 已取消
}
