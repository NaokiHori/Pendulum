#include <stdio.h>
#include <stdlib.h>
#include <errno.h>
#include "common.h"


void *common_calloc(const size_t count, const size_t size){
  errno = 0;
  void *ptr = calloc(count, size);
  if(NULL == ptr){
    perror("");
    exit(EXIT_FAILURE);
  }
  return ptr;
}

void common_free(void *ptr){
  free(ptr);
}

FILE *common_fopen(const char * restrict path, const char * restrict mode){
  errno = 0;
  FILE *stream = fopen(path, mode);
  if(NULL == stream){
    perror(path);
    exit(EXIT_FAILURE);
  }
  return stream;
}

int common_fclose(FILE *stream){
  fclose(stream);
  return 0;
}

