import 'dart:ffi';
import 'dart:io';
import 'package:ffi/ffi.dart';
import 'types.dart';

class _Bindings {
  DynamicLibrary lexibook;
  void Function(int) initLogger;
  Pointer<Utf8> Function() lastErrorMessage;
  Pointer<Void> Function(Pointer<Utf8>) parseFile;
  Pointer<Void> Function(Pointer<Utf8>) parseString;
  Pointer<Void> Function(Pointer<Utf8>) openGlossary;
  int Function(Pointer<Void>, Pointer<Utf8>)  saveGlossary;
  Pointer<Void> Function(Pointer<Utf8>) fromJson;
  void Function(Pointer<Void>) soundSystemFree;
  void Function(Pointer<StringList>) stringListFree;
  Pointer<StringList> Function(Pointer<Void>, int, int) generateWords;
  Pointer<StringList> Function(Pointer<Void>, Pointer<StringList>) applyTransformations;
  Pointer<Utf8> Function(Pointer<Void>, Pointer<Utf8>) getIpa;
  int Function(Pointer<Void>, Pointer<Utf8>) saveFile;

  _Bindings() {
    lexibook = Platform.isAndroid
        ? DynamicLibrary.open("liblexibook_ffi.so")
        : (Platform.isWindows
            ? DynamicLibrary.open("lexibook_ffi.dll")
            : DynamicLibrary.process());

    initLogger = lexibook
        .lookup<NativeFunction<lexibook_init_logger_t>>('lexibook_init_logger')
        .asFunction();

    lastErrorMessage = lexibook
        .lookup<NativeFunction<last_error_message_t>>(
            'lexibook_last_error_message')
        .asFunction();

    parseFile = lexibook
        .lookup<NativeFunction<parse_func_t>>('lexibook_parse_file')
        .asFunction();

    parseString = lexibook
        .lookup<NativeFunction<parse_func_t>>('lexibook_parse_string')
        .asFunction();

    fromJson = lexibook
        .lookup<NativeFunction<parse_func_t>>('lexibook_from_json')
        .asFunction();

    openGlossary = lexibook
        .lookup<NativeFunction<parse_func_t>>('lexibook_open_glossary')
        .asFunction();

    saveGlossary = lexibook
        .lookup<NativeFunction<lexibook_save_file_t>>('lexibook_save_glossary')
        .asFunction();
    soundSystemFree = lexibook
        .lookup<NativeFunction<lexibook_sound_system_free_t>>(
            'lexibook_sound_system_free')
        .asFunction();

    stringListFree = lexibook
        .lookup<NativeFunction<lexibook_string_list_free_t>>(
            'lexibook_string_list_free')
        .asFunction();

    generateWords = lexibook
        .lookup<NativeFunction<lexibook_generate_words_t>>(
            'lexibook_generate_words')
        .asFunction();

    applyTransformations = lexibook
        .lookup<NativeFunction<lexibook_apply_transformations_t>>(
            'lexibook_apply_transformations')
        .asFunction();

    getIpa = lexibook
        .lookup<NativeFunction<lexibook_get_ipa_t>>(
            'lexibook_get_ipa')
        .asFunction();

    saveFile = lexibook
        .lookup<NativeFunction<lexibook_save_file_t>>(
            'lexibook_sound_system_save_file')
        .asFunction();
  }
}

_Bindings _cachedBindings;
_Bindings get bindings => _cachedBindings ??= _Bindings();
