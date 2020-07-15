import 'dart:async';
import 'dart:io';

import 'package:file_chooser/file_chooser.dart';
import 'package:file_picker/file_picker.dart';

import 'file_picker.dart';

Future<String> getPath({String fileExtension}) async {
  if (Platform.isAndroid || Platform.isIOS || Platform.isFuchsia) {
    return await selectFilesMobile(fileExtension: fileExtension);
  } else {
    return await selectFilesDesktop(fileExtension: fileExtension);
  }
}

Future<String> selectFilesDesktop({String fileExtension}) async {
  FileChooserResult file = await showOpenPanel(
      allowedFileTypes: (parseExtension(fileExtension) == null)
          ? null
          : [
              FileTypeFilterGroup(
                  label: 'files', fileExtensions: parseExtension(fileExtension))
            ]);
  return file.paths[0];
}

Future<String> selectFilesMobile({String fileExtension}) async {
  return await FilePicker.getFilePath(
      type: FileType.custom, allowedExtensions: parseExtension(fileExtension));
}

dynamic parseExtension(String fileExtension) {
  return (fileExtension != null &&
          fileExtension.replaceAll(',', '').trim().isNotEmpty)
      ? fileExtension.split(',').map<String>((e) => e.trim()).toList()
      : null;
}
