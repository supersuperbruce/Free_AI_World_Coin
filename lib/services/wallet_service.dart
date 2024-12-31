import 'package:bip39/bip39.dart' as bip39;
import 'package:web3dart/web3dart.dart';
import 'package:flutter_secure_storage/flutter_secure_storage.dart';
import 'package:convert/convert.dart';
import '../models/wallet.dart';
import 'package:flutter/foundation.dart';

class WalletService extends ChangeNotifier {
  final FlutterSecureStorage _secureStorage = const FlutterSecureStorage();
  static const String _mnemonicKey = 'wallet_mnemonic';
  static const String _privateKeyKey = 'wallet_private_key';

  // 创建新钱包
  Future<WalletModel> createWallet() async {
    // 生成随机助记词
    final mnemonic = bip39.generateMnemonic();
    // 从助记词生成私钥种子
    final seed = bip39.mnemonicToSeed(mnemonic);
    // 创建凭证
    final credentials = EthPrivateKey.fromHex(hex.encode(seed.sublist(0, 32)));
    // 获取钱包地址
    final address = credentials.address;

    // 创建钱包实例
    final wallet = WalletModel(
      address: address.hex,
      privateKey: credentials.privateKeyInt.toRadixString(16),
      mnemonic: mnemonic,
      balance: 0.0,
    );

    // 安全存储助记词和私钥
    await _secureStorage.write(key: _mnemonicKey, value: mnemonic);
    await _secureStorage.write(
        key: _privateKeyKey,
        value: credentials.privateKeyInt.toRadixString(16));

    return wallet;
  }

  // 检查是否已存在钱包
  Future<bool> hasWallet() async {
    final mnemonic = await _secureStorage.read(key: _mnemonicKey);
    return mnemonic != null;
  }

  // 获取当前钱包
  Future<WalletModel?> getCurrentWallet() async {
    final mnemonic = await _secureStorage.read(key: _mnemonicKey);
    final privateKey = await _secureStorage.read(key: _privateKeyKey);

    if (mnemonic == null || privateKey == null) {
      return null;
    }

    final credentials = EthPrivateKey.fromHex(privateKey);
    final address = credentials.address;

    return WalletModel(
      address: address.hex,
      privateKey: privateKey,
      mnemonic: mnemonic,
    );
  }

  // 添加通知方法
  void notifyWalletChanged() {
    notifyListeners();
  }
}
