import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import '../provider/message_provider.dart';

class Message {
  String text;
  bool isSentByUser;
  DateTime timestamp;

  Message({required this.text, required this.isSentByUser, required this.timestamp});
}

class ChatScreen extends StatefulWidget {
  final String contactName;

  ChatScreen({required this.contactName});

  @override
  _ChatScreenState createState() => _ChatScreenState();
}

class _ChatScreenState extends State<ChatScreen> {
  final TextEditingController _textController = TextEditingController();
  final ScrollController _scrollController = ScrollController();

  @override
  void initState() {
    super.initState();
    // Connect to the WebSocket server when the screen is initialized
    // The URL is hardcoded for now, but should be configurable
    Provider.of<MessageProvider>(context, listen: false).connect('ws://localhost:8080');
  }

  @override
  void dispose() {
    _textController.dispose();
    _scrollController.dispose();
    super.dispose();
  }

  void _sendMessage(MessageProvider messageProvider) {
    if (_textController.text.isNotEmpty) {
      messageProvider.sendMessage(_textController.text);
      _textController.clear();
      // Scroll to the bottom after a short delay to allow the list to update
      Future.delayed(Duration(milliseconds: 50), () {
        _scrollController.animateTo(
          _scrollController.position.maxScrollExtent,
          duration: Duration(milliseconds: 300),
          curve: Curves.easeOut,
        );
      });
    }
  }

  @override
  Widget build(BuildContext context) {
    final messageProvider = Provider.of<MessageProvider>(context);

    return Scaffold(
      appBar: AppBar(
        title: Text(widget.contactName),
      ),
      body: Column(
        children: [
          Expanded(
            child: ListView.builder(
              controller: _scrollController,
              itemCount: messageProvider.messages.length,
              itemBuilder: (context, index) {
                final message = messageProvider.messages[index];
                return _buildMessageBubble(message);
              },
            ),
          ),
          _buildMessageComposer(messageProvider),
        ],
      ),
    );
  }

  Widget _buildMessageBubble(Message message) {
    final bool isSentByUser = message.isSentByUser;
    return Align(
      alignment: isSentByUser ? Alignment.centerRight : Alignment.centerLeft,
      child: Container(
        margin: EdgeInsets.symmetric(vertical: 4.0, horizontal: 8.0),
        padding: EdgeInsets.symmetric(vertical: 10.0, horizontal: 15.0),
        decoration: BoxDecoration(
          color: isSentByUser ? Colors.blueAccent : Colors.grey[300],
          borderRadius: BorderRadius.only(
            topLeft: Radius.circular(15.0),
            topRight: Radius.circular(15.0),
            bottomLeft: isSentByUser ? Radius.circular(15.0) : Radius.circular(0.0),
            bottomRight: isSentByUser ? Radius.circular(0.0) : Radius.circular(15.0),
          ),
        ),
        child: Column(
          crossAxisAlignment: isSentByUser ? CrossAxisAlignment.end : CrossAxisAlignment.start,
          children: [
            Text(
              message.text,
              style: TextStyle(
                color: isSentByUser ? Colors.white : Colors.black,
              ),
            ),
            SizedBox(height: 5.0),
            Text(
              '${message.timestamp.hour}:${message.timestamp.minute}',
              style: TextStyle(
                color: isSentByUser ? Colors.white70 : Colors.black54,
                fontSize: 10.0,
              ),
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildMessageComposer(MessageProvider messageProvider) {
    return Container(
      padding: EdgeInsets.all(8.0),
      decoration: BoxDecoration(
        color: Theme.of(context).cardColor,
        boxShadow: [
          BoxShadow(
            color: Colors.grey.withOpacity(0.5),
            spreadRadius: 1,
            blurRadius: 5,
            offset: Offset(0, 3), // changes position of shadow
          ),
        ],
      ),
      child: Row(
        children: [
          Expanded(
            child: TextField(
              controller: _textController,
              decoration: InputDecoration.collapsed(
                hintText: 'Send a message...',
              ),
              onSubmitted: (_) => _sendMessage(messageProvider),
            ),
          ),
          IconButton(
            icon: Icon(Icons.send, color: Theme.of(context).primaryColor),
            onPressed: () => _sendMessage(messageProvider),
          ),
        ],
      ),
    );
  }
}