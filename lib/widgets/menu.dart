import 'package:flutter/material.dart';
import 'package:flutter_neumorphic/flutter_neumorphic.dart';
import 'package:lexibook/helpers/display.dart';
import 'package:hive/hive.dart';
import 'package:hive_flutter/hive_flutter.dart';
import 'package:lexibook/extensions/string.dart';

class Menu extends StatelessWidget {
  final _themes = ['system', 'light', 'dark'];

  Widget build(BuildContext context) => Drawer(
        child: Container(
          color: NeumorphicTheme.baseColor(context),
          child: Column(
            mainAxisAlignment: MainAxisAlignment.start,
            mainAxisSize: MainAxisSize.max,
            children: [
              ConstrainedBox(
                constraints: BoxConstraints.tightFor(
                    height: NeumorphicAppBar.toolbarHeight),
                child: NeumorphicAppBar(
                  title: Text(
                    'Menu',
                    style: Display.mainText(context),
                  ),
                  leading: NeumorphicCloseButton(),
                  actions: <Widget>[
                    NeumorphicBackButton(forward: true),
                  ],
                ),
              ),
              Padding(
                padding: EdgeInsets.all(12),
                child: Column(
                  mainAxisSize: MainAxisSize.max,
                  children: <Widget>[
                    Text(
                      "Theme",
                      style: Display.mainText(context),
                    ),
                    Padding(
                      padding: EdgeInsets.all(12),
                      child: _themeSwitcher(context),
                    ),
                  ],
                ),
              ),
              Spacer(),
            ],
          ),
        ),
      );

  Widget _themeSwitcher(BuildContext context) => ValueListenableBuilder(
      valueListenable: Hive.box('settings').listenable(),
      builder: (context, box, widget) {
        var idx = _themes.indexOf(box.get('themeMode', defaultValue: 'system'));

        return NeumorphicToggle(
          selectedIndex: idx,
          thumb: Neumorphic(
            style: NeumorphicStyle(
              boxShape: NeumorphicBoxShape.roundRect(
                  BorderRadius.all(Radius.circular(12))),
            ),
          ),
          children: _themes.map((theme) {
            var name = theme.capitalize();
            return ToggleElement(
              background: Center(
                child: Text(name, style: Display.mainText(context)),
              ),
              foreground: Center(
                child: Text(name, style: Display.mainText(context)),
              ),
            );
          }).toList(),
          onChanged: (value) {
            box.put('themeMode', _themes[value]);
          },
        );
      });
}
