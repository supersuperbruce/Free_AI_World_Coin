import 'package:flutter/material.dart';
import 'swap_page.dart';
import 'send_page.dart';
import 'receive_page.dart';

class ActionsPage extends StatelessWidget {
  const ActionsPage({super.key});

  void _navigateToAction(BuildContext context, String action) {
    switch (action) {
      case '兑换':
        Navigator.push(
          context,
          MaterialPageRoute(builder: (context) => const SwapPage()),
        );
        break;
      case '发送':
        Navigator.push(
          context,
          MaterialPageRoute(builder: (context) => const SendPage()),
        );
        break;
      case '接收':
        Navigator.push(
          context,
          MaterialPageRoute(builder: (context) => ReceivePage()),
        );
        break;
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('操作'),
      ),
      body: GridView.count(
        padding: const EdgeInsets.all(16),
        crossAxisCount: 3,
        mainAxisSpacing: 16,
        crossAxisSpacing: 16,
        children: [
          _ActionButton(
            icon: Icons.swap_horiz,
            label: '兑换',
            color: Colors.blue,
            onPressed: () => _navigateToAction(context, '兑换'),
          ),
          _ActionButton(
            icon: Icons.send,
            label: '发送',
            color: Colors.orange,
            onPressed: () => _navigateToAction(context, '发送'),
          ),
          _ActionButton(
            icon: Icons.call_received,
            label: '接收',
            color: Colors.teal,
            onPressed: () => _navigateToAction(context, '接收'),
          ),
        ],
      ),
    );
  }
}

class _ActionButton extends StatelessWidget {
  final IconData icon;
  final String label;
  final Color color;
  final VoidCallback onPressed;

  const _ActionButton({
    required this.icon,
    required this.label,
    required this.color,
    required this.onPressed,
  });

  @override
  Widget build(BuildContext context) {
    return Material(
      color: Colors.transparent,
      child: InkWell(
        onTap: onPressed,
        borderRadius: BorderRadius.circular(12),
        child: Container(
          decoration: BoxDecoration(
            color: color.withOpacity(0.1),
            borderRadius: BorderRadius.circular(12),
          ),
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              Icon(icon, color: color, size: 32),
              const SizedBox(height: 8),
              Text(
                label,
                style: TextStyle(color: color),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
