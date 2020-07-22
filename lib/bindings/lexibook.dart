import 'dart:ffi';
import 'package:ffi/ffi.dart';
import 'bindings.dart';

enum LogLevel {
  Trace,
  Debug,
  Info,
  Warn,
  Error,
}

enum MonoSyllableRepartition {
  Always,
  Mostly,
  Frequent,
  LessFrequent,
  Rare,
  Never,
}

void initLogger(LogLevel level) {
  bindings.init_logger(level.index);
}

class SoundSystem {
  Pointer<Void> _ptr = nullptr.cast<Void>();
  SoundSystem._(Pointer<Void> ptr) {
    _ptr = ptr;
  }

  static SoundSystem parseFile(String input) {
    var utf8Input = Utf8.toUtf8(input);
    var ptr = bindings.parse_file(utf8Input);
    free(utf8Input);
    if (ptr.address == nullptr.address) {
      var error = bindings.last_error_message();
      throw Utf8.fromUtf8(error);
    }
    return SoundSystem._(ptr);
  }

  static SoundSystem parseString(String input) {
    var utf8Input = Utf8.toUtf8(input);
    var ptr = bindings.parse_string(utf8Input);
    free(utf8Input);
    if (ptr.address == nullptr.address) {
      var error = bindings.last_error_message();
      throw Utf8.fromUtf8(error);
    }
    return SoundSystem._(ptr);
  }

  List<String> generateWords(int numbers, MonoSyllableRepartition frequency) {
    var list = bindings.generate_words(_ptr, numbers, frequency.index);
    var items = list.ref.items.cast<IntPtr>();

    List<String> result = List(list.ref.length);
    for (int i = 0; i < list.ref.length; i++) {
      var string = Utf8.fromUtf8(Pointer.fromAddress(items[i]));
      result[i] = string;
    }
    bindings.string_list_free(list);
    return result;
  }

  void save(String filename) {
    var utf8Input = Utf8.toUtf8(filename);
    var result = bindings.save_file(_ptr, utf8Input);
    free(utf8Input);
    if (result == 0) {
      var lastErrorPtr = bindings.last_error_message();
      var lastError = Utf8.fromUtf8(lastErrorPtr);
      free(lastErrorPtr);
      throw Exception(lastError);
    }
  }

  void close() {
    bindings.sound_system_free(_ptr);
  }
}
