#if !defined(COMMON_H)
#define COMMON_H

#include <stdio.h>
#include <stdlib.h>

extern void *common_calloc(const size_t count, const size_t size);
extern void common_free(void *ptr);

extern FILE *common_fopen(const char * restrict path, const char * restrict mode);
extern int common_fclose(FILE *stream);

#endif // COMMON_H
