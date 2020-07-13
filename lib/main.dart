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
          primarySwatch: Colors.blue,
        ),
        home: NeumorphicTheme(
            usedTheme: UsedTheme.LIGHT,
            theme: NeumorphicThemeData(
              baseColor: Color(0xFFFFFFFF),
              lightSource: LightSource.topLeft,
              depth: 10,
            ),
            darkTheme: NeumorphicThemeData(
              baseColor: Color(0xFF3E3E3E),
              lightSource: LightSource.topLeft,
              depth: 6,
            ),
            child: HomeScreen()),
      );
}
