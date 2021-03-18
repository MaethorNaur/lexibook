import 'dart:async';
import 'dart:io';
import 'package:path/path.dart' as Path;
import 'package:optional/optional.dart';
import 'package:file_selector/file_selector.dart';
import 'package:file_picker/file_picker.dart';

Future<Optional<String>> saveFilePath(String fileExtension) {
  if (Platform.isAndroid || Platform.isIOS || Platform.isFuchsia) {
    return _saveFileMobile(fileExtension: fileExtension);
  } else {
    return _saveFileDesktop(fileExtension: fileExtension);
  }
}

Future<Optional<String>> openFilePath(String fileExtension) {
  if (Platform.isAndroid || Platform.isIOS || Platform.isFuchsia) {
    return _selectFileMobile(fileExtension: fileExtension);
  } else {
    return _selectFileDesktop(fileExtension: fileExtension);
  }
}

Future<Optional<String>> _selectFileDesktop({String fileExtension}) {
  XTypeGroup group =
      XTypeGroup(label: "wgl", extensions: _parseExtension(fileExtension));
  return openFile(acceptedTypeGroups: [group])
      .then((file) => Optional.ofNullable(file).map((file) => file.path));
}

Future<Optional<String>> _saveFileDesktop({String fileExtension}) {
  XTypeGroup group =
      XTypeGroup(label: "wgl", extensions: _parseExtension(fileExtension));
 
  return getSavePath(acceptedTypeGroups: [group])
      .then((file) => Optional.ofNullable(file));
}

Future<Optional<String>> _selectFileMobile({String fileExtension}) {
  return FilePicker.platform
      .pickFiles(
        type: FileType.any,
      )
      .then((result) => Optional.ofNullable(result)
          .map((result) => result.files.single.path));
}

Future<Optional<String>> _saveFileMobile({String fileExtension}) {
  return FilePicker.platform.getDirectoryPath().then((path) {
    print(path);
    return Optional.ofNullable(
        path != null ? Path.join(path, "file.wgl") : null);
  });
}

dynamic _parseExtension(String fileExtension) {
  return (fileExtension != null &&
          fileExtension.replaceAll(',', '').trim().isNotEmpty)
      ? fileExtension.split(',').map<String>((e) => e.trim()).toList()
      : null;
}
