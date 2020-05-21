import 'package:flutter/material.dart';
import 'package:meta/meta.dart';
import 'package:flutter_neumorphic/flutter_neumorphic.dart';

class WidgetInputScreen extends StatelessWidget {
  const WidgetInputScreen({
    @required this.wglCallback,
    @required this.initText,
  });

  final String initText;
  final ValueChanged<String> wglCallback;

  @override
  Widget build(BuildContext context) => Neumorphic(
         boxShape: NeumorphicBoxShape.roundRect(BorderRadius.circular(12)),
         style: NeumorphicStyle(
           shape: NeumorphicShape.flat,
         ),

        margin: EdgeInsets.all(16).copyWith(top: 8),
        padding: EdgeInsets.all(10),
        child: TextField(
          maxLines: 10,
          keyboardType: TextInputType.multiline,
          decoration: InputDecoration(border: InputBorder.none),
          controller: TextEditingController(text: initText),
          onChanged: wglCallback,
        ),
      );
}
