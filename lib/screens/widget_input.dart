import 'package:flutter/material.dart';
import 'package:meta/meta.dart';
import 'package:flutter_neumorphic/flutter_neumorphic.dart';

class WidgetInputScreen extends StatefulWidget {
  final String initText;
  final ValueChanged<String> wglCallback;
  const WidgetInputScreen({
    Key key,
    @required this.wglCallback,
    @required this.initText,
  }) : super(key: key);
  _WidgetInputScreen createState() => _WidgetInputScreen();
}

class _WidgetInputScreen extends State<WidgetInputScreen> {
  final TextEditingController controller = TextEditingController();
  @override
  void initState() {
    super.initState();
    controller.text = widget.initText;
  }

  @override
  Widget build(BuildContext context) => Neumorphic(
        style: NeumorphicStyle(
          shape: NeumorphicShape.flat,
          boxShape: NeumorphicBoxShape.roundRect(BorderRadius.circular(12)),
        ),
        margin: EdgeInsets.all(16).copyWith(top: 8),
        padding: EdgeInsets.all(10),
        child: TextField(
          maxLines: 10,
          keyboardType: TextInputType.multiline,
          decoration: InputDecoration(border: InputBorder.none),
          controller: controller,
          onChanged: widget.wglCallback,
        ),
      );
}
