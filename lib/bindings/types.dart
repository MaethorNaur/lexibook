import 'dart:ffi';
import 'package:ffi/ffi.dart';


class StringList extends Struct {
  Pointer<Pointer<Utf8>> items;
  @Int64()
  int length;
}

typedef lexibook_init_logger_t = Void Function(Uint8);
typedef last_error_message_t = Pointer<Utf8> Function();
typedef parse_func_t = Pointer<Void> Function(Pointer<Utf8>);
typedef lexibook_sound_system_free_t = Void Function(Pointer<Void>);
typedef lexibook_string_list_free_t = Void Function(Pointer<StringList>);
typedef lexibook_generate_words_t = Pointer<StringList> Function( Pointer<Void>, Uint32, Uint8);
typedef lexibook_save_file_t = Uint8 Function( Pointer<Void>, Pointer<Utf8>);

