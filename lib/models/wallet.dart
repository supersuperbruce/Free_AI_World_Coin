
class WalletModel {
  final String address;
  final String privateKey;
  final String? mnemonic;
  final double balance;

  WalletModel({
    required this.address,
    required this.privateKey,
    this.mnemonic,
    this.balance = 0.0,
  });

  factory WalletModel.fromJson(Map<String, dynamic> json) {
    return WalletModel(
      address: json['address'] as String,
      privateKey: json['privateKey'] as String,
      mnemonic: json['mnemonic'] as String?,
      balance: (json['balance'] as num?)?.toDouble() ?? 0.0,
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'address': address,
      'privateKey': privateKey,
      'mnemonic': mnemonic,
      'balance': balance,
    };
  }
}