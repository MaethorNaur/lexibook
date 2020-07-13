import 'package:flutter/material.dart';
import 'package:flutter_neumorphic/flutter_neumorphic.dart';
import 'bindings/lexibook.dart';
import 'screens/home.dart';
import 'package:flutter/foundation.dart' as Foundation;

void main() {
  if (Foundation.kReleaseMode) {
    initLogger(LogLevel.Info);
  } else {
    initLogger(LogLevel.Trace);
  }
  runApp(LexibookApp());
}

class LexibookApp extends StatelessWidget {
  @override
  Widget build(BuildContext context) => MaterialApp(
        title: 'Lexibook',
        theme: ThemeData(
          primarySwatch: Colors.green,
        ),
        home: NeumorphicTheme(
            usedTheme: UsedTheme.LIGHT,
            darkTheme: NeumorphicThemeData(
              baseColor: Colors.grey[800],
              accentColor: Colors.green,
              lightSource: LightSource.topLeft,
              depth: 4,
              intensity: 0.3,
            ),
            theme: NeumorphicThemeData(
              baseColor: Colors.grey[200],
              accentColor: Colors.cyan,
              lightSource: LightSource.topLeft,
              depth: 6,
              intensity: 0.5,
            ),
            child: HomeScreen()),
      );
}
