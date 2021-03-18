import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_neumorphic/flutter_neumorphic.dart';
import 'package:optional/optional.dart';
import 'package:lexibook/file_picker.dart' as FilePicker;

import 'package:lexibook/helpers/display.dart';
import 'package:lexibook/bindings/lexibook.dart';
import 'package:lexibook/widgets/frequency.dart';
import 'package:lexibook/widgets/menu.dart';
import 'package:eva_icons_flutter/eva_icons_flutter.dart';

class HomeScreen extends StatefulWidget {
  @override
  _HomeScreenState createState() => _HomeScreenState();
}

class _HomeScreenState extends State<HomeScreen> {
  List<String> _words = [];
  double _numbers = 10;
  SoundSystem _soundSystem;
  MonoSyllableRepartition _frequency = MonoSyllableRepartition.LessFrequent;
  String _filename = "";
  final ScrollController _scrollController = ScrollController();

  void _parseFile(String file) {
    setState(() {
      if (_soundSystem != null) {
        _soundSystem.close();
      }
      try {
        _soundSystem = SoundSystem.parseFile(file);
        _filename = file;
        _words = [];
      } catch (e) {
        print("Error: $e");
      }
    });
  }

  void _openGlossary(String file) {
    setState(() {
      if (_soundSystem != null) {
        _soundSystem.close();
      }
      try {
        _soundSystem = SoundSystem.openGlossary(file);
        _filename = file;
        _words = [];
      } catch (e) {
        print("Error: $e");
      }
    });
  }

  void _generateWords() {
    setState(() {
      _words = _soundSystem.generateWords(_numbers.toInt(), _frequency);
      _words = _soundSystem.applyTransformations(_words);
    });
  }

  @override
  void dispose() {
    _scrollController.dispose();
    super.dispose();
  }

  Widget _iconText(BuildContext context, {IconData icon, String text}) => Wrap(
        crossAxisAlignment: WrapCrossAlignment.center,
        children: [
          Icon(icon),
          Text(
            text,
            style: Display.mainText(context),
          ),
        ],
      );

  @override
  Widget build(BuildContext context) {
    SystemChrome.setSystemUIOverlayStyle(SystemUiOverlayStyle(
      statusBarColor: Colors.transparent,
      statusBarBrightness: Brightness.light,
      statusBarIconBrightness: Brightness.dark,
      systemNavigationBarIconBrightness: Brightness.dark,
      systemNavigationBarColor: NeumorphicTheme.baseColor(context),
    ));
    return Scaffold(
      backgroundColor: NeumorphicTheme.baseColor(context),
      appBar: NeumorphicAppBar(
        title: Text(
          "Lexibook",
          style: Display.mainText(context),
        ),
      ),
      endDrawer: Menu(),
      body: SafeArea(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.spaceEvenly,
          mainAxisSize: MainAxisSize.max,
          children: <Widget>[
            Padding(
              padding: EdgeInsets.all(10),
              child: Row(
                mainAxisAlignment: MainAxisAlignment.start,
                // spacing: 24.0,
                // runSpacing: 24.0,
                children: <Widget>[
                  NeumorphicButton(
                    margin: EdgeInsets.only(top: 12),
                    onPressed: () async {
                      Optional<String> maybePath =
                          await FilePicker.openFilePath('wgl');
                      maybePath.ifPresent((path) => _parseFile(path));
                    },
                    style: Display.flatRounded(),
                    padding: const EdgeInsets.all(12.0),
                    child: _iconText(context,
                        icon: EvaIcons.downloadOutline, text: "Load file"),
                  ),
                  Spacer(),
                  NeumorphicButton(
                    margin: EdgeInsets.only(top: 12),
                    onPressed: () async {
                      Optional<String> maybePath =
                          await FilePicker.openFilePath('glr');
                      maybePath.ifPresent((path) => _openGlossary(path));
                    },
                    style: Display.flatRounded(),
                    padding: const EdgeInsets.all(12.0),
                    child: _iconText(context,
                        icon: EvaIcons.downloadOutline, text: "Load glossary"),
                  ),
                  Spacer(),
                  NeumorphicButton(
                    margin: EdgeInsets.only(top: 12),
                    onPressed: _soundSystem != null
                        ? () async {
                            Optional<String> maybePath =
                                await FilePicker.saveFilePath('glr');
                            maybePath
                                .ifPresent((path) => _soundSystem.save(path));
                          }
                        : null,
                    style: Display.flatRounded(),
                    padding: const EdgeInsets.all(12.0),
                    child: _iconText(context,
                        icon: EvaIcons.saveOutline, text: "Save file"),
                  ),
                  Spacer(),
                  Text(
                    "$_filename",
                    style: Display.mainText(context),
                  ),
                ],
              ),
            ),
            Padding(
              padding: EdgeInsets.all(10),
              child: Row(
                mainAxisSize: MainAxisSize.max,
                children: <Widget>[
                  Text(
                    "Words: ",
                    style: Display.mainText(context),
                  ),
                  Flexible(
                    child: NeumorphicSlider(
                      min: 10,
                      max: 100,
                      value: _numbers,
                      onChanged: (value) {
                        setState(() {
                          _numbers = value;
                        });
                      },
                    ),
                  ),
                  Text(
                    "${_numbers.toInt()}",
                    style: Display.mainText(context),
                  ),
                ],
              ),
            ),
            FrequencyWidget(
              defaultValue: _frequency,
              callback: (value) => setState(() => _frequency = value),
            ),
            Expanded(
              child: Scrollbar(
                isAlwaysShown: true,
                controller: _scrollController,
                child: ListView.builder(
                    controller: _scrollController,
                    padding: const EdgeInsets.all(8),
                    itemCount: _words.length,
                    itemBuilder: (BuildContext context, int index) {
                      return Neumorphic(
                        style: Display.flatRounded(),
                        margin: EdgeInsets.all(16).copyWith(top: 8),
                        padding: EdgeInsets.all(16),
                        child: Center(
                          child: Text(
                            _words[index],
                            style: Display.mainText(context),
                          ),
                        ),
                      );
                    }),
              ),
            ),
          ],
        ),
      ),
      floatingActionButton: NeumorphicButton(
        onPressed: _soundSystem != null ? _generateWords : null,
        style: Display.flatRounded(),
        child: NeumorphicIcon(
          Icons.keyboard_return,
          style: NeumorphicStyle(
              color: NeumorphicTheme.of(context).current.accentColor),
        ),
      ),
    );
  }
}
