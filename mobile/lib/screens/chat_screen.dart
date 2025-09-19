
import 'package:flutter/material.dart';
import 'package:http/http.dart' as http;

class ChatScreen extends StatelessWidget {
  Future<String> _getJitsiToken() async {
    // In a real application, you would get the auth token from your app's state management
    final authToken = 'your_auth_token'; // Replace with a valid auth token

    final response = await http.get(
      Uri.parse('http://127.0.0.1:3000/sfu/token'),
      headers: {
        'Authorization': 'Bearer $authToken',
      },
    );

    if (response.statusCode == 200) {
      return response.body;
    } else {
      throw Exception('Failed to get Jitsi token');
    }
  }

  void _startCall() async {
    try {
      final token = await _getJitsiToken();
      // Now you would use this token to join the Jitsi meeting.
      // This part requires a Jitsi client SDK (e.g., jitsi_meet_flutter_sdk).
      print('Got Jitsi token: $token');
    } catch (e) {
      print('Error starting call: $e');
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text('Chat'),
        actions: [
          IconButton(
            icon: Icon(Icons.video_call),
            onPressed: _startCall,
          ),
        ],
      ),
      body: Center(
        child: Text('Chat UI'),
      ),
    );
  }
}
