#include "../src/ffi/lexibook.h"
#include <stdio.h>

int main(int argc, const char *argv[]) {
  if (argc < 2) {
    return 1;
  }
  void *ptr = lexibook_parse_file(argv[1]);
  if (ptr == NULL) {
    char *error = lexibook_last_error_message();
    if (error != NULL) {
      printf("%s\n", error);
      free(error);
    }
    return 1;
  }

  lexibook_api_StringList words =
      lexibook_generate_words(ptr, 10, LessFrequent);
  printf("Words:\n");
  for (int i = 0; i < words.length; i++) {
    printf("%s\n", words.items[i]);
  }

  lexibook_string_list_free(words);
  lexibook_sound_system_free(ptr);
  return 0;
}
