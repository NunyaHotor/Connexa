
import 'package:flutter/material.dart';

class ChangeNotifierProvider<T extends ChangeNotifier> extends StatefulWidget {
  final Widget child;
  final T Function() create;

  ChangeNotifierProvider({required this.create, required this.child});

  static T of<T extends ChangeNotifier>(BuildContext context, {bool listen = true}) {
    final provider = listen
        ? context.dependOnInheritedWidgetOfExactType<_InheritedChangeNotifier<T>>()
        : context.findAncestorWidgetOfExactType<_InheritedChangeNotifier<T>>();
    return provider!.notifier;
  }

  @override
  _ChangeNotifierProviderState<T> createState() => _ChangeNotifierProviderState<T>();
}

class _ChangeNotifierProviderState<T extends ChangeNotifier> extends State<ChangeNotifierProvider<T>> {
  late T notifier;

  @override
  void initState() {
    super.initState();
    notifier = widget.create();
  }

  @override
  void dispose() {
    notifier.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return _InheritedChangeNotifier(
      notifier: notifier,
      child: widget.child,
    );
  }
}

class _InheritedChangeNotifier<T extends ChangeNotifier> extends InheritedWidget {
  final T notifier;

  _InheritedChangeNotifier({required this.notifier, required Widget child}) : super(child: child);

  @override
  bool updateShouldNotify(_InheritedChangeNotifier<T> oldWidget) {
    return notifier != oldWidget.notifier;
  }
}

class Consumer<T extends ChangeNotifier> extends StatelessWidget {
  final Widget Function(BuildContext context, T value, Widget? child) builder;
  final Widget? child;

  Consumer({required this.builder, this.child});

  @override
  Widget build(BuildContext context) {
    return builder(
      context,
      ChangeNotifierProvider.of<T>(context),
      child,
    );
  }
}
