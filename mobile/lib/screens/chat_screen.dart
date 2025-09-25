import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import '../provider/message_provider.dart';
import 'package:image_picker/image_picker.dart';
import 'package:file_picker/file_picker.dart';
import 'package:mime/mime.dart' as mime;
import 'dart:typed_data';

class Message {
  String text;
  bool isSentByUser;
  DateTime timestamp;
  DateTime? expiresAt; // null means no local expiry

  Message({required this.text, required this.isSentByUser, required this.timestamp, this.expiresAt});
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
  int? _selectedTtlSeconds; // null = off

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
      messageProvider.sendMessage(_textController.text, ttlSeconds: _selectedTtlSeconds);
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

  Future<void> _openAttachmentSheet(MessageProvider messageProvider) async {
    showModalBottomSheet(
      context: context,
      builder: (ctx) {
        return SafeArea(
          child: Wrap(
            children: [
              ListTile(
                leading: Icon(Icons.camera_alt),
                title: Text('Take Photo'),
                onTap: () async {
                  Navigator.pop(ctx);
                  final ImagePicker picker = ImagePicker();
                  final XFile? photo = await picker.pickImage(source: ImageSource.camera);
                  if (photo != null) {
                    final bytes = await photo.readAsBytes();
                    final mt = mime.lookupMimeType(photo.name) ?? 'image/jpeg';
                    await messageProvider.sendMedia(
                      data: bytes,
                      fileName: photo.name,
                      mimeType: mt,
                      ttlSeconds: _selectedTtlSeconds,
                    );
                  }
                },
              ),
              ListTile(
                leading: Icon(Icons.photo),
                title: Text('Pick Image'),
                onTap: () async {
                  Navigator.pop(ctx);
                  final ImagePicker picker = ImagePicker();
                  final XFile? image = await picker.pickImage(source: ImageSource.gallery);
                  if (image != null) {
                    final bytes = await image.readAsBytes();
                    final mt = mime.lookupMimeType(image.name) ?? 'image/jpeg';
                    await messageProvider.sendMedia(
                      data: bytes,
                      fileName: image.name,
                      mimeType: mt,
                      ttlSeconds: _selectedTtlSeconds,
                    );
                  }
                },
              ),
              ListTile(
                leading: Icon(Icons.attach_file),
                title: Text('Pick File'),
                onTap: () async {
                  Navigator.pop(ctx);
                  final result = await FilePicker.platform.pickFiles(withData: true);
                  if (result != null && result.files.isNotEmpty) {
                    final file = result.files.first;
                    final Uint8List? bytes = file.bytes;
                    if (bytes != null) {
                      final mt = mime.lookupMimeType(file.name) ?? 'application/octet-stream';
                      await messageProvider.sendMedia(
                        data: bytes,
                        fileName: file.name,
                        mimeType: mt,
                        ttlSeconds: _selectedTtlSeconds,
                      );
                    }
                  }
                },
              ),
            ],
          ),
        );
      },
    );
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
          IconButton(
            icon: Icon(Icons.attach_file, color: Theme.of(context).primaryColor),
            onPressed: () => _openAttachmentSheet(messageProvider),
          ),
          PopupMenuButton<int?>(
            icon: Icon(
              Icons.timer,
              color: _selectedTtlSeconds == null ? Colors.grey : Theme.of(context).primaryColor,
            ),
            onSelected: (value) {
              setState(() {
                _selectedTtlSeconds = value;
              });
            },
            itemBuilder: (context) => [
              const PopupMenuItem<int?>(value: null, child: Text('Off')),
              const PopupMenuDivider(),
              const PopupMenuItem<int?>(value: 10, child: Text('10 seconds')),
              const PopupMenuItem<int?>(value: 60, child: Text('1 minute')),
              const PopupMenuItem<int?>(value: 3600, child: Text('1 hour')),
              const PopupMenuItem<int?>(value: 86400, child: Text('1 day')),
            ],
            tooltip: 'Disappearing message timer',
          ),
          SizedBox(width: 4),
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