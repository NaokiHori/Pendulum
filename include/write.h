#if !defined(WRITE_H)
#define WRITE_H

#include <stddef.h>
#include "pendulum.h"

typedef struct writer_t_ writer_t;

typedef struct {
  // constructor
  writer_t * (*construct)(
      const double rate,
      const pendulum_t *pendulum
      );
  // destructor
  void (*destruct)(
      writer_t *writer
      );
  // getter
  double (*get_next_time)(
      const writer_t *writer
      );
  // update display, output mass information to files
  void (*write)(
      const double time,
      writer_t *writer,
      const pendulum_t *pendulum
  );
} write_methods_t;

extern write_methods_t write_methods;

#endif // WRITE_H
