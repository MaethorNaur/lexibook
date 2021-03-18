import 'package:flutter/material.dart';
import 'package:flutter_neumorphic/flutter_neumorphic.dart';
import 'package:lexibook/widgets/menu.dart';
import 'package:lexibook/helpers/display.dart';

class Settings extends StatelessWidget {
  Widget build(BuildContext context) => Scaffold(
      backgroundColor: NeumorphicTheme.baseColor(context),
      appBar: NeumorphicAppBar(
        title: Text(
          "Lexibook",
          style: Display.mainText(context),
        ),
      ),
      body: SafeArea(
        child: Menu(),
      ));
}
