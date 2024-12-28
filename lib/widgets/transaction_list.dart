import 'package:flutter/material.dart';

class TransactionList extends StatelessWidget {
  const TransactionList({super.key});

  @override
  Widget build(BuildContext context) {
    return ListView.builder(
      itemCount: 0, // 暂时没有交易记录
      itemBuilder: (context, index) {
        return const ListTile(
          title: Text('暂无交易记录'),
        );
      },
    );
  }
}
