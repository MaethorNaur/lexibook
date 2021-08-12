import 'dart:io';
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
var _bindings = Bindings(Platform.isAndroid
        ? DynamicLibrary.open("liblexibook_ffi.so")
        : (Platform.isWindows
            ? DynamicLibrary.open("lexibook_ffi.dll")
            : DynamicLibrary.process()));
void initLogger(LogLevel level) {
  _bindings.lexibook_init_logger(level.index);
}

class SoundSystem {
  Pointer<Void> _ptr = nullptr.cast<Void>();
  SoundSystem._(Pointer<Void> ptr) {
    _ptr = ptr;
  }

  static SoundSystem parseFile(String input) {
    var utf8Input = input.toNativeUtf8().cast<Int8>();
    var ptr = _bindings.lexibook_parse_file(utf8Input);
    calloc.free(utf8Input);
    if (ptr.address == nullptr.address) {
      var error = _bindings.lexibook_last_error_message();
      throw error.cast<Utf8>().toDartString();
    }
    return SoundSystem._(ptr);
  }

  static SoundSystem parseString(String input) {
    var utf8Input = input.toNativeUtf8().cast<Int8>();
    var ptr = _bindings.lexibook_parse_string(utf8Input);
    calloc.free(utf8Input);
    if (ptr.address == nullptr.address) {
      var error = _bindings.lexibook_last_error_message();
      throw error.cast<Utf8>().toDartString();
    }
    return SoundSystem._(ptr);
  }

  List<String> generateWords(int numbers, MonoSyllableRepartition frequency) {
    var list = _bindings.lexibook_generate_words(_ptr, numbers, frequency.index);
    var items = list.ref.items.cast<IntPtr>();

    List<String> result = List.filled(list.ref.length,"");
    for (int i = 0; i < list.ref.length; i++) {
      var string = Pointer.fromAddress(items[i]).cast<Utf8>().toDartString();
      result[i] = string;
    }
    _bindings.lexibook_string_list_free(list);
    return result;
  }

  void save(String filename) {
    var utf8Input = filename.toNativeUtf8().cast<Int8>();
    var result = _bindings.lexibook_sound_system_save_file(_ptr, utf8Input);
    calloc.free(utf8Input);
    if (result == 0) {
      var lastErrorPtr = _bindings.lexibook_last_error_message();
      var lastError = lastErrorPtr.cast<Utf8>().toDartString();
      calloc.free(lastErrorPtr);
      throw Exception(lastError);
    }
  }

  void close() {
    _bindings.lexibook_sound_system_free(_ptr);
  }
}
