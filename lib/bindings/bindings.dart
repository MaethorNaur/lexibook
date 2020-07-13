import 'dart:ffi';
import 'dart:io';
import 'package:ffi/ffi.dart';
import 'types.dart';

class _Bindings {
  DynamicLibrary lexibook;
  void Function(int) init_logger;
  Pointer<Utf8> Function() last_error_message;

  Pointer<Void> Function(Pointer<Utf8>) parse_file;
  Pointer<Void> Function(Pointer<Utf8>) parse_string;
  void Function(Pointer<Void>) sound_system_free;
  void Function(Pointer<StringList>) string_list_free;
  Pointer<StringList> Function(Pointer<Void>, int, int) generate_words;

  _Bindings() {
    lexibook = Platform.isAndroid
        ? DynamicLibrary.open("liblexibook_ffi.so")
        :(Platform.isWindows ?
          DynamicLibrary.open("lexibook_ffi.dll")
        : DynamicLibrary.process());

    init_logger = lexibook
        .lookup<NativeFunction<lexibook_init_logger_t>>('lexibook_init_logger')
        .asFunction();
    last_error_message = lexibook
        .lookup<NativeFunction<last_error_message_t>>(
            'lexibook_last_error_message')
        .asFunction();
    parse_file = lexibook
        .lookup<NativeFunction<parse_func_t>>('lexibook_parse_file')
        .asFunction();
    parse_string = lexibook
        .lookup<NativeFunction<parse_func_t>>('lexibook_parse_string')
        .asFunction();
    sound_system_free = lexibook
        .lookup<NativeFunction<lexibook_sound_system_free_t>>(
            'lexibook_sound_system_free')
        .asFunction();
    string_list_free = lexibook
        .lookup<NativeFunction<lexibook_string_list_free_t>>(
            'lexibook_string_list_free')
        .asFunction();
    generate_words = lexibook
        .lookup<NativeFunction<lexibook_generate_words_t>>(
            'lexibook_generate_words')
        .asFunction();
  }
}
_Bindings _cachedBindings;
_Bindings get bindings => _cachedBindings ??= _Bindings();
