
import 'package:flutter/material.dart';
import '../db/real_database_helper.dart';
import '../screens/contact_screen.dart';

class ContactProvider extends ChangeNotifier {
  final dbHelper = RealDatabaseHelper.instance;
  List<Contact> _contacts = [];

  List<Contact> get contacts => _contacts;

  Future<void> loadContacts() async {
    final loadedContacts = await dbHelper.queryAllContacts();
    _contacts = loadedContacts;
    notifyListeners();
  }

  Future<void> addContact(Contact contact) async {
    await dbHelper.insertContact(contact);
    await loadContacts();
  }

  Future<void> deleteContact(int id) async {
    await dbHelper.deleteContact(id);
    await loadContacts();
  }
}
