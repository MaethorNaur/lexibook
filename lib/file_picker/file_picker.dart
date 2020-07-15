import 'file_picker_io.dart';

/// FilePickerCross allows you to select files on any of Flutters platforms.
class FilePickerCross {
  final String fileExtension;

  FilePickerCross({this.fileExtension = ''});

  /// Shows a dialog for selecting a file.
  Future<String> pick() async {
    return await getPath(fileExtension: fileExtension);
  }
}
