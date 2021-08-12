import 'dart:async';
import 'dart:io';
import 'package:path/path.dart' as Path;
import 'package:file_selector_platform_interface/file_selector_platform_interface.dart';

Future<String?> saveFilePath()  {
    return FileSelectorPlatform.instance.getSavePath(
          acceptedTypeGroups:  [XTypeGroup(
      label: 'text',
      extensions: ['txt', 'json'],
    )]);
}

Future<String?> openFilePath() {
return FileSelectorPlatform.instance.openFile(
          acceptedTypeGroups:  [XTypeGroup(
      label: 'wgl',
      extensions: ['wgl'],
    )])
      .then((file) => file?.path);
}