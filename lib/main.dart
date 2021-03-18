import 'package:flutter/material.dart';
import 'package:flutter_neumorphic/flutter_neumorphic.dart';
import 'bindings/lexibook.dart';
import 'screens/home.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/services.dart' show rootBundle;
import 'package:hive/hive.dart';
import 'package:hive_flutter/hive_flutter.dart';

void main() async {
  await Hive.initFlutter();
  await Hive.openBox('settings');
  LicenseRegistry.addLicense(() async* {
    final license = await rootBundle.loadString('google_fonts/OFL.txt');
    yield LicenseEntryWithLineBreaks(['google_fonts'], license);
  });
  if (kReleaseMode) {
    initLogger(LogLevel.Warn);
  } else {
    initLogger(LogLevel.Info);
  }
  runApp(LexibookApp());
}

class LexibookApp extends StatelessWidget {
  @override
  Widget build(BuildContext context) => ValueListenableBuilder(
      valueListenable: Hive.box('settings').listenable(),
      builder: (context, box, widget) {
        var themeMode;
        switch (box.get('themeMode', defaultValue: 'system')) {
          case 'system':
            themeMode = ThemeMode.system;
            break;
          case 'light':
            themeMode = ThemeMode.light;
            break;
          case 'dark':
            themeMode = ThemeMode.dark;
            break;
        }
        return NeumorphicApp(
          title: 'Lexibook',
          home: NeumorphicTheme(
              themeMode: themeMode,
              theme: NeumorphicThemeData(
                defaultTextColor: Color(0xFF3E3E3E),
                baseColor: Color(0xFFDDE6E8),
              ),
              darkTheme: neumorphicDefaultDarkTheme.copyWith(
                defaultTextColor: Colors.white70,
              ),
              child: HomeScreen()),
        );
      });
}
