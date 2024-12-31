import 'package:flutter/material.dart';

class WalletPage extends StatelessWidget {
  const WalletPage({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('FAIC 钱包'),
        actions: [
          IconButton(
            icon: const Icon(Icons.qr_code_scanner),
            onPressed: () {
              // TODO: 实现扫码功能
            },
          ),
        ],
      ),
      body: Column(
        children: [
          // 钱包地址卡片
          _buildAddressCard(),
          // 代币余额列表
          _buildTokenBalanceList(),
        ],
      ),
    );
  }

  Widget _buildAddressCard() {
    return Card(
      margin: const EdgeInsets.all(16),
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          children: [
            Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: [
                const Text('钱包地址'),
                IconButton(
                  icon: const Icon(Icons.copy),
                  onPressed: () {
                    // TODO: 复制地址功能
                  },
                ),
              ],
            ),
            const Text('0x...'), // 这里显示实际地址
          ],
        ),
      ),
    );
  }

  Widget _buildTokenBalanceList() {
    return Expanded(
      child: ListView.builder(
        itemCount: 1, // 暂时只显示FAIC代币
        itemBuilder: (context, index) {
          return ListTile(
            leading: const CircleAvatar(child: Text('F')),
            title: const Text('FAIC'),
            subtitle: const Text('Free AI Chain'),
            trailing: const Text('0.00'), // 这里显示实际余额
          );
        },
      ),
    );
  }
}