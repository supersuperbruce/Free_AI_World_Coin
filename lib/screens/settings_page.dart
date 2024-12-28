import 'package:flutter/material.dart';

class SettingsPage extends StatelessWidget {
  const SettingsPage({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('设置'),
      ),
      body: ListView(
        children: [
          ListTile(
            leading: const Icon(Icons.security),
            title: const Text('安全设置'),
            onTap: () {
              // TODO: 导航到安全设置页面
            },
          ),
          ListTile(
            leading: const Icon(Icons.backup),
            title: const Text('备份助记词'),
            onTap: () {
              // TODO: 导航到备份助记词页面
            },
          ),
          ListTile(
            leading: const Icon(Icons.info),
            title: const Text('关于'),
            onTap: () {
              // TODO: 导航到关于页面
            },
          ),
        ],
      ),
    );
  }
}
