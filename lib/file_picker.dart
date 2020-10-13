import 'dart:async';
import 'dart:io';
import 'package:path/path.dart' as Path;
import 'package:optional/optional.dart';
import 'package:file_chooser/file_chooser.dart';
import 'package:file_picker/file_picker.dart';

Future<Optional<String>> saveFilePath(String fileExtension)  {
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
  return showOpenPanel(
          allowedFileTypes: (_parseExtension(fileExtension) == null)
              ? null
              : [
                  FileTypeFilterGroup(
                      label: 'file',
                      fileExtensions: _parseExtension(fileExtension))
                ])
      .then((file) =>
          Optional.ofNullable(file.paths.isEmpty ? null : file.paths.first));
}

Future<Optional<String>> _saveFileDesktop({String fileExtension}) {
  return showSavePanel(
          allowedFileTypes: (_parseExtension(fileExtension) == null)
              ? null
              : [
                  FileTypeFilterGroup(
                      label: 'files',
                      fileExtensions: _parseExtension(fileExtension))
                ])
      .then((file) =>
          Optional.ofNullable(file.paths.isEmpty ? null : file.paths.first));
}

Future<Optional<String>> _selectFileMobile({String fileExtension}) {
  return FilePicker.getFilePath(
    type: FileType.any,
  ).then((path) => Optional.ofNullable(path));
}

Future<Optional<String>> _saveFileMobile({String fileExtension}) {
  return FilePicker.getDirectoryPath().then((path) {
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
