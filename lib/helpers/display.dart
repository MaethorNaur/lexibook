import 'package:flutter/material.dart';
import 'package:flutter_neumorphic/flutter_neumorphic.dart';
import 'package:google_fonts/google_fonts.dart';

class Display {
  static TextStyle mainText(BuildContext context) =>
      GoogleFonts.montserrat(textStyle: TextStyle(color: NeumorphicTheme.defaultTextColor(context)));

  static NeumorphicStyle flatRounded() => NeumorphicStyle(
      shape: NeumorphicShape.flat,
      boxShape: NeumorphicBoxShape.roundRect(BorderRadius.circular(8)));
}
