import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_neumorphic/flutter_neumorphic.dart';
import 'package:lexibook/bindings/lexibook.dart';
import 'frequency.dart';
import 'package:lexibook/helpers/display.dart';
import 'package:lexibook/file_picker/file_picker.dart';

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
  final FilePickerCross filePickerCross = FilePickerCross(fileExtension: 'wgl');

  void _parseFile(String file) {
    setState(() {
      if (_soundSystem != null) {
        _soundSystem.close();
      }
      try {
        _soundSystem = SoundSystem.parseFile(file);
        _filename = file;
      } catch (e) {
        print("Error: $e");
      }
    });
  }

  void _generateWords() {
    setState(() {
      _words = _soundSystem.generateWords(_numbers.toInt(), _frequency);
    });
  }

  @override
  void dispose() {
    _scrollController.dispose();
    super.dispose();
  }

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
      body: SafeArea(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.spaceEvenly,
          mainAxisSize: MainAxisSize.max,
          children: <Widget>[
            Wrap(
              spacing: 24.0,
              runSpacing: 24.0,
              children: <Widget>[
                NeumorphicButton(
                  margin: EdgeInsets.only(top: 12),
                  onPressed: () {
                    NeumorphicTheme.of(context).themeMode =
                        NeumorphicTheme.isUsingDark(context)
                            ? ThemeMode.light
                            : ThemeMode.dark;
                  },
                  style: Display.flatRounded(),
                  padding: const EdgeInsets.all(12.0),
                  child: Text(
                    "Toggle Theme",
                    style: Display.mainText(context),
                  ),
                ),
                NeumorphicButton(
                    margin: EdgeInsets.only(top: 12),
                    onPressed: () async {
                      String path = await filePickerCross.pick();
                      _parseFile(path);
                    },
                    style: Display.flatRounded(),
                    padding: const EdgeInsets.all(12.0),
                    child: Text(
                      "Load file",
                      style: Display.mainText(context),
                    )),
                Text(
                  "$_filename",
                  style: Display.mainText(context),
                ),
              ],
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
          style: NeumorphicStyle(color: NeumorphicTheme.of(context).current.accentColor),
        ),
      ),
    );
  }
}
