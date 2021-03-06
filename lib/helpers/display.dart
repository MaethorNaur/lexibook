import 'package:flutter/material.dart';
import 'package:flutter_neumorphic/flutter_neumorphic.dart';
import 'package:google_fonts/google_fonts.dart';

class Display {
  static Color _textColor(BuildContext context) =>
      NeumorphicTheme.isUsingDark(context) ? Colors.white : Colors.black;

  static TextStyle mainText(BuildContext context) =>
      GoogleFonts.montserrat(textStyle: TextStyle(color: _textColor(context)));

  static NeumorphicStyle flatRounded() => NeumorphicStyle(
      shape: NeumorphicShape.flat,
      boxShape: NeumorphicBoxShape.roundRect(BorderRadius.circular(8)));
}
