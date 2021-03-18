import 'package:flutter/material.dart';
import 'package:meta/meta.dart';
import 'package:flutter_neumorphic/flutter_neumorphic.dart';
import 'package:lexibook/bindings/lexibook.dart';
import 'package:lexibook/helpers/display.dart';

class FrequencyWidget extends StatelessWidget {
  final ValueChanged<MonoSyllableRepartition> callback;
  final MonoSyllableRepartition defaultValue;
  const FrequencyWidget({
    Key key,
    @required this.defaultValue,
    @required this.callback,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    List<ToggleElement> children =
        MonoSyllableRepartition.values.map((value) {
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
      return ToggleElement(
        background: Center(
          child: Text(
            name,
            style: Display.mainText(context),
          ),
        ),
        foreground: Center(
          child: Text(
            name,
            style: Display.mainText(context),
          ),
        ),
      );
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
          NeumorphicToggle(
              selectedIndex: defaultValue.index,
              thumb: Neumorphic(
                style: NeumorphicStyle(
                  boxShape: NeumorphicBoxShape.roundRect(
                      BorderRadius.all(Radius.circular(12))),
                ),
              ),
              onChanged: (value) {
                callback(MonoSyllableRepartition.values[value]);
              },
              children: children),
        ],
      ),
    );
  }
}
