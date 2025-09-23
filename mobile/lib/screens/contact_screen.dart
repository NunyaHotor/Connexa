import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import '../provider/contact_provider.dart';
import '../provider/message_provider.dart';
import 'chat_screen.dart';

class Contact {
  int? id;
  String name;
  String initial;

  Contact({this.id, required this.name, required this.initial});

  Map<String, dynamic> toMap() {
    return {
      'id': id,
      'name': name,
      'initial': initial,
    };
  }
}

class ContactScreen extends StatefulWidget {
  @override
  _ContactScreenState createState() => _ContactScreenState();
}

class _ContactScreenState extends State<ContactScreen> {
  final TextEditingController _nameController = TextEditingController();
  final TextEditingController _initialController = TextEditingController();

  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addPostFrameCallback((_) {
      Provider.of<ContactProvider>(context, listen: false).loadContacts();
    });
  }

  void _addContact(BuildContext context) async {
    if (_nameController.text.isNotEmpty && _initialController.text.isNotEmpty) {
      await Provider.of<ContactProvider>(context, listen: false).addContact(
        Contact(name: _nameController.text, initial: _initialController.text),
      );
      _nameController.clear();
      _initialController.clear();
      Navigator.of(context).pop(); // Close the dialog
    }
  }

  void _deleteContact(BuildContext context, int id) async {
    await Provider.of<ContactProvider>(context, listen: false).deleteContact(id);
  }

  void _showAddContactDialog(BuildContext context) {
    showDialog(
      context: context,
      builder: (BuildContext context) {
        return AlertDialog(
          title: Text('Add New Contact'),
          content: Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              TextField(
                controller: _nameController,
                decoration: InputDecoration(labelText: 'Contact Name'),
              ),
              TextField(
                controller: _initialController,
                decoration: InputDecoration(labelText: 'Initial'),
                maxLength: 1,
              ),
            ],
          ),
          actions: <Widget>[
            TextButton(
              child: Text('Cancel'),
              onPressed: () {
                _nameController.clear();
                _initialController.clear();
                Navigator.of(context).pop();
              },
            ),
            ElevatedButton(
              child: Text('Add'),
              onPressed: () => _addContact(context),
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
        title: Text('Contacts'),
      ),
      body: Consumer<ContactProvider>(
        builder: (context, contactProvider, child) {
          return ListView.builder(
            itemCount: contactProvider.contacts.length,
            itemBuilder: (context, index) {
              final contact = contactProvider.contacts[index];
              return Dismissible(
                key: Key(contact.id.toString()),
                direction: DismissDirection.endToStart,
                onDismissed: (direction) {
                  _deleteContact(context, contact.id!);
                  ScaffoldMessenger.of(context).showSnackBar(
                    SnackBar(content: Text('${contact.name} dismissed')),
                  );
                },
                background: Container(
                  color: Colors.red,
                  alignment: Alignment.centerRight,
                  padding: EdgeInsets.symmetric(horizontal: 20.0),
                  child: Icon(Icons.delete, color: Colors.white),
                ),
                child: InkWell(
                  onTap: () {
                    Navigator.push(
                      context,
                      MaterialPageRoute(
                        builder: (context) => ChangeNotifierProvider(
                          create: (context) => MessageProvider(),
                          child: ChatScreen(contactName: contact.name),
                        ),
                      ),
                    );
                  },
                  child: ListTile(
                    leading: CircleAvatar(
                      child: Text(contact.initial),
                    ),
                    title: Text(contact.name),
                  ),
                ),
              );
            },
          );
        },
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: () => _showAdd-ContactDialog(context),
        child: Icon(Icons.add),
      ),
    );
  }
}