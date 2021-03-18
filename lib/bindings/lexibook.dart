import 'dart:ffi';
import 'package:ffi/ffi.dart';
import 'bindings.dart';
import 'dart:convert';
import 'types.dart';

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
  bindings.initLogger(level.index);
}

class SoundSystem {
  Pointer<Void> _ptr = nullptr.cast<Void>();

  SoundSystem._(Pointer<Void> ptr) {
    _ptr = ptr;
  }

  static SoundSystem parseFile(String input) {
    var utf8Input = input.toNativeUtf8();
    var ptr = bindings.parseFile(utf8Input);
    malloc.free(utf8Input);
    if (ptr.address == nullptr.address) {
      var lastErrorPtr = bindings.lastErrorMessage();
      var lastError = lastErrorPtr.toDartString();
      malloc.free(lastErrorPtr);
      throw Exception(lastError);
    }
    return SoundSystem._(ptr);
  }

  static SoundSystem openGlossary(String input) {
    var utf8Input = input.toNativeUtf8();
    var ptr = bindings.openGlossary(utf8Input);
    malloc.free(utf8Input);
    if (ptr.address == nullptr.address) {
      var lastErrorPtr = bindings.lastErrorMessage();
      var lastError = lastErrorPtr.toDartString();
      malloc.free(lastErrorPtr);
      throw Exception(lastError);
    }
    return SoundSystem._(ptr);
  }


  static SoundSystem fromJson(Map<String, dynamic> data) {
    var utf8Input = json.encode(data).toNativeUtf8();
    var ptr = bindings.fromJson(utf8Input);
    malloc.free(utf8Input);
    if (ptr.address == nullptr.address) {
      var lastErrorPtr = bindings.lastErrorMessage();
      var lastError = lastErrorPtr.toDartString();
      malloc.free(lastErrorPtr);
      throw Exception(lastError);
    }
    return SoundSystem._(ptr);
  }

  static SoundSystem parseString(String input) {
    var utf8Input = input.toNativeUtf8();
    var ptr = bindings.parseString(utf8Input);
    malloc.free(utf8Input);
    if (ptr.address == nullptr.address) {
      var lastErrorPtr = bindings.lastErrorMessage();
      var lastError = lastErrorPtr.toDartString();
      malloc.free(lastErrorPtr);
      throw Exception(lastError);
    }
    return SoundSystem._(ptr);
  }

  List<String> generateWords(int numbers, MonoSyllableRepartition frequency) {
    var list = bindings.generateWords(_ptr, numbers, frequency.index);
    List<String> result = List.filled(list.ref.length, "");
    for (int i = 0; i < list.ref.length; i++) {
      result[i] = list.ref.items[i].toDartString();
    }
    bindings.stringListFree(list);
    return result;
  }

  List<String> applyTransformations(List<String> words) {
    var listPtr = malloc.call<StringList>();
    listPtr.ref.length = words.length;
    Pointer<Pointer<Utf8>> items = malloc.call<Pointer<Utf8>>(words.length);

    for (int i = 0; i < words.length; i++) {
      items[i] = words[i].toNativeUtf8();
    }
    listPtr.ref.items = items;
    var resultList = bindings.applyTransformations(_ptr, listPtr);
    if (resultList.address == nullptr.address) {
      var lastErrorPtr = bindings.lastErrorMessage();
      var lastError = lastErrorPtr.toDartString();
      malloc.free(lastErrorPtr);
      throw Exception(lastError);
    }
    List<String> result = List.filled(resultList.ref.length, "");
    for (int i = 0; i < resultList.ref.length; i++) {
      result[i] = resultList.ref.items[i].toDartString();
    }
    malloc.free(listPtr);
    bindings.stringListFree(resultList);
    return result;
  }

  String getIpa(String word) {
    var utf8Input = word.toNativeUtf8();
    var ptr = bindings.getIpa(_ptr, utf8Input);
    malloc.free(utf8Input);
    if (ptr.address == nullptr.address) {
      var lastErrorPtr = bindings.lastErrorMessage();
      var lastError = lastErrorPtr.toDartString();
      malloc.free(lastErrorPtr);
      throw Exception(lastError);
    }
    var result = ptr.toDartString();
    malloc.free(ptr);
    return result;
  }

  void save(String filename) {
    var utf8Input = filename.toNativeUtf8();
    var result = bindings.saveGlossary(_ptr, utf8Input);
    malloc.free(utf8Input);
    if (result == 0) {
      var lastErrorPtr = bindings.lastErrorMessage();
      var lastError = lastErrorPtr.toDartString();
      malloc.free(lastErrorPtr);
      throw Exception(lastError);
    }
  }

  void close() {
    bindings.soundSystemFree(_ptr);
  }
}
