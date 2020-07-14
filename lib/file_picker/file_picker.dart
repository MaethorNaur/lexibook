import 'file_picker_io.dart';

/// FilePickerCross allows you to select files on any of Flutters platforms.
class FilePickerCross {
  final FileTypeCross type;
  final String fileExtension;

  FilePickerCross({this.type = FileTypeCross.any, this.fileExtension = ''});

  /// Shows a dialog for selecting a file.
  Future<String> pick() async {
    return await getPath(type: type, fileExtension: fileExtension);
  }
}

/// Supported file types
enum FileTypeCross { image, video, audio, any, custom }
