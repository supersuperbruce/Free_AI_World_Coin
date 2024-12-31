import 'package:flutter/foundation.dart';

@immutable
class Token {
  final String symbol;    // 代币符号，如 "FAIC"
  final String name;      // 代币名称，如 "Free AI Chain"
  final String address;   // 代币合约地址
  final int decimals;     // 精度
  final String? iconUrl;  // 图标URL
  final double balance;   // 余额

  const Token({
    required this.symbol,
    required this.name,
    required this.address,
    required this.decimals,
    this.iconUrl,
    this.balance = 0.0,
  });

  Token copyWith({
    String? symbol,
    String? name,
    String? address,
    int? decimals,
    String? iconUrl,
    double? balance,
  }) {
    return Token(
      symbol: symbol ?? this.symbol,
      name: name ?? this.name,
      address: address ?? this.address,
      decimals: decimals ?? this.decimals,
      iconUrl: iconUrl ?? this.iconUrl,
      balance: balance ?? this.balance,
    );
  }

  @override
  bool operator ==(Object other) {
    if (identical(this, other)) return true;
    return other is Token && other.address == address;
  }

  @override
  int get hashCode => address.hashCode;
}