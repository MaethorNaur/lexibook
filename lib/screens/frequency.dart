import 'package:flutter/material.dart';
import 'package:meta/meta.dart';
import 'package:flutter_neumorphic/flutter_neumorphic.dart';
import 'package:lexibook/bindings/lexibook.dart';

class FrequencyWidget extends StatefulWidget {
  final ValueChanged<MonoSyllableRepartition> callback;
  final MonoSyllableRepartition defaultValue;
  const FrequencyWidget({
    Key key,
    @required this.defaultValue,
    @required this.callback,
  }) : super(key: key);

  _FrequencyState createState() => _FrequencyState();
}

class _FrequencyState extends State<FrequencyWidget> {
  Color _textColor(BuildContext context) =>
      NeumorphicTheme.isUsingDark(context) ? Colors.white : Colors.black;

  int repartition;

  @override
  void initState() {
    super.initState();
    repartition = widget.defaultValue.index;
  }

  @override
  Widget build(BuildContext context) {
    Iterable<Widget> children = MonoSyllableRepartition.values.map((value) {
      String name;
      switch (value) {
        case MonoSyllableRepartition.LessFrequent:
          name = "Less Frequent";
          break;
        case MonoSyllableRepartition.Mostly:
          name = "Mostly";
          break;
        case MonoSyllableRepartition.Never:
          name = "Never";
          break;
        case MonoSyllableRepartition.Rare:
          name = "Rare";
          break;
        case MonoSyllableRepartition.Always:
          name = "Always";
          break;
        case MonoSyllableRepartition.Frequent:
          name = "Frequent";
          break;
      }
      return NeumorphicRadio(
          groupValue: repartition,
          value: value.index,
          onChanged: (value) {
            setState(() => repartition = value);
            widget.callback(MonoSyllableRepartition.values[value]);
          },
          boxShape: NeumorphicBoxShape.roundRect(BorderRadius.circular(8)),
          padding: const EdgeInsets.all(12.0),
          child: Text(
            name,
            style: TextStyle(color: _textColor(context)),
          ));
    }).toList();

    return Padding(
      padding: EdgeInsets.all(10),
      child: Row(
        mainAxisSize: MainAxisSize.max,
        mainAxisAlignment: MainAxisAlignment.spaceEvenly,
        children: <Widget>[
          Text(
            "Frequency",
            style: TextStyle(color: _textColor(context)),
          ),
          SizedBox(width: 12),
        ]..addAll(children),
      ),
    );
  }
}
