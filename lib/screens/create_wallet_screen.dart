import 'package:flutter/material.dart';
import '../services/wallet_service.dart';
import '../models/wallet.dart';

class CreateWalletScreen extends StatefulWidget {
  const CreateWalletScreen({super.key});

  @override
  State<CreateWalletScreen> createState() => _CreateWalletScreenState();
}

class _CreateWalletScreenState extends State<CreateWalletScreen> {
  final WalletService _walletService = WalletService();
  bool _isLoading = false;
  WalletModel? _wallet;

  Future<void> _createWallet() async {
    setState(() {
      _isLoading = true;
    });

    try {
      final wallet = await _walletService.createWallet();
      setState(() {
        _wallet = wallet;
      });
      // 显示助记词备份页面
      if (mounted) {
        _showBackupMnemonicDialog(wallet.mnemonic!);
      }
    } catch (e) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text('创建钱包失败: ${e.toString()}')),
        );
      }
    } finally {
      setState(() {
        _isLoading = false;
      });
    }
  }

  void _showBackupMnemonicDialog(String mnemonic) {
    showDialog(
      context: context,
      barrierDismissible: false,
      builder: (BuildContext context) {
        return AlertDialog(
          title: const Text('备份助记词'),
          content: Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              const Text(
                '请将以下助记词安全备份，它可以用来恢复您的钱包。\n'
                '警告：切勿将助记词分享给他人！',
                style: TextStyle(color: Colors.red),
              ),
              const SizedBox(height: 16),
              Container(
                padding: const EdgeInsets.all(16),
                decoration: BoxDecoration(
                  border: Border.all(color: Colors.grey),
                  borderRadius: BorderRadius.circular(8),
                ),
                child: Text(
                  mnemonic,
                  style: const TextStyle(fontSize: 16),
                  textAlign: TextAlign.center,
                ),
              ),
            ],
          ),
          actions: [
            TextButton(
              onPressed: () {
                Navigator.of(context).pop();
                // TODO: 导航到主钱包页面
                // 我们稍后会实现这个功能
              },
              child: const Text('我已安全备份'),
            ),
          ],
        );
      },
    );
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('创建钱包'),
      ),
      body: Center(
        child: _isLoading
            ? const CircularProgressIndicator()
            : Column(
                mainAxisAlignment: MainAxisAlignment.center,
                children: [
                  const Text(
                    '欢迎使用FAIC钱包',
                    style: TextStyle(fontSize: 24, fontWeight: FontWeight.bold),
                  ),
                  const SizedBox(height: 32),
                  ElevatedButton(
                    onPressed: _createWallet,
                    child: const Text('创建新钱包'),
                  ),
                  const SizedBox(height: 16),
                  TextButton(
                    onPressed: () {
                      // TODO: 导航到导入钱包页面
                      // 我们稍后会实现这个功能
                    },
                    child: const Text('导入已有钱包'),
                  ),
                ],
              ),
      ),
    );
  }
}
