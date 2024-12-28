import 'package:flutter/material.dart';
import 'package:faic_wallet/widgets/transaction_list.dart';

class TransactionPage extends StatelessWidget {
  const TransactionPage({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('交易记录'),
      ),
      body: const TransactionList(),
    );
  }
}
