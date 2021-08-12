import 'package:flutter/material.dart';
import 'package:meta/meta.dart';
import 'package:flutter_neumorphic/flutter_neumorphic.dart';
import 'package:lexibook/bindings/lexibook.dart';
import 'package:lexibook/helpers/display.dart';

class FrequencyWidget extends StatefulWidget {
  final ValueChanged<MonoSyllableRepartition> callback;
  final MonoSyllableRepartition defaultValue;
  const FrequencyWidget({
    Key? key,
    required this.defaultValue,
    required this.callback,
  }) : super(key: key);

  _FrequencyState createState() => _FrequencyState();
}

class _FrequencyState extends State<FrequencyWidget> {
  int? repartition;

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
          onChanged: (int? value) {
            if(value != null) {
            setState(() => repartition = value);
            widget.callback(MonoSyllableRepartition.values[value]);
}
          },
          style: NeumorphicRadioStyle(
            boxShape: NeumorphicBoxShape.roundRect(BorderRadius.circular(8)),
          ),
          padding: const EdgeInsets.all(12.0),
          child: Text(
            name,
            style: Display.mainText(context),
          ));
    }).toList();

    return Padding(
      padding: EdgeInsets.all(10),
      child: Wrap(
        spacing: 12.0,
        runSpacing: 8.0,
        children: <Widget>[
          Text(
            "Frequency",
            style: Display.mainText(context),
          ),
          SizedBox(width: 12),
        ]..addAll(children),
      ),
    );
  }
}
