import 'dart:async';
import 'dart:io';

import 'package:file_chooser/file_chooser.dart';
import 'package:file_picker/file_picker.dart';

Future<String> getFilePath(String fileExtension) async {
  if (Platform.isAndroid || Platform.isIOS || Platform.isFuchsia) {
    return await _selectFilesMobile(fileExtension: fileExtension);
  } else {
    return await _selectFilesDesktop(fileExtension: fileExtension);
  }
}

Future<String> _selectFilesDesktop({String fileExtension}) async {
  FileChooserResult file = await showOpenPanel(
      allowedFileTypes: (_parseExtension(fileExtension) == null)
          ? null
          : [
              FileTypeFilterGroup(
                  label: 'files', fileExtensions: _parseExtension(fileExtension))
            ]);
  return file.paths[0];
}

Future<String> _selectFilesMobile({String fileExtension}) async {
  return await FilePicker.getFilePath(
      type: FileType.custom, allowedExtensions: _parseExtension(fileExtension));
}

dynamic _parseExtension(String fileExtension) {
  return (fileExtension != null &&
          fileExtension.replaceAll(',', '').trim().isNotEmpty)
      ? fileExtension.split(',').map<String>((e) => e.trim()).toList()
      : null;
}
