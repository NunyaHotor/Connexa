
import '../screens/contact_screen.dart';

class MockDatabaseHelper {
  static final MockDatabaseHelper instance = MockDatabaseHelper._privateConstructor();
  MockDatabaseHelper._privateConstructor();

  final List<Contact> _contacts = [
    Contact(name: 'Alice', initial: 'A'),
    Contact(name: 'Bob', initial: 'B'),
  ];

  Future<List<Contact>> queryAllContacts() async {
    return _contacts;
  }

  Future<int> insertContact(Contact contact) async {
    _contacts.add(contact);
    return 1;
  }

  Future<int> deleteContact(String name) async {
    _contacts.removeWhere((contact) => contact.name == name);
    return 1;
  }
}
