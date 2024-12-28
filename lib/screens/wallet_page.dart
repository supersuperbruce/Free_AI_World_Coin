import 'package:flutter/material.dart';
import 'package:faic_wallet/widgets/wallet_card.dart';
import 'package:faic_wallet/widgets/action_buttons.dart';

class WalletPage extends StatelessWidget {
  const WalletPage({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('FAIC 钱包'),
        actions: [
          IconButton(
            icon: const Icon(Icons.qr_code),
            onPressed: () {
              // TODO: 实现扫码功能
            },
          ),
        ],
      ),
      body: SingleChildScrollView(
        child: Column(
          children: [
            const WalletCard(), // 显示钱包地址和余额
            const SizedBox(height: 20),
            const ActionButtons(), // 买入、卖出、发送等按钮
          ],
        ),
      ),
    );
  }
}
