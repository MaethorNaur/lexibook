import 'package:flutter/material.dart';
import 'package:flutter_neumorphic/flutter_neumorphic.dart';
import 'bindings/lexibook.dart';
import 'screens/home.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/services.dart' show rootBundle;

void main() {
  LicenseRegistry.addLicense(() async* {
    final license = await rootBundle.loadString('google_fonts/OFL.txt');
    yield LicenseEntryWithLineBreaks(['google_fonts'], license);
  });
  if (kReleaseMode) {
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
            themeMode: ThemeMode.system,
            darkTheme: NeumorphicThemeData(
              baseColor: Colors.grey[800],
              accentColor: Colors.green,
            ),
            theme: NeumorphicThemeData(
              baseColor: Colors.grey[200],
              accentColor: Colors.cyan,
            ),
            child: HomeScreen()),
      );
}
