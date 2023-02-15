#if !defined(LOGGING_H)
#define LOGGING_H

#include <stddef.h>
#include "pendulum.h"

typedef struct logger_t_ logger_t;

typedef struct {
  // constructor
  logger_t * (*construct)(
      const double rate,
      const pendulum_t *pendulum
      );
  // destructor
  void (*destruct)(
      logger_t *logger
      );
  // getter
  double (*get_next_time)(
      const logger_t *logger
      );
  // output values to be monitored
  void (*write)(
      const double time,
      const double dt,
      logger_t *logger,
      const pendulum_t *pendulum
  );
} logging_methods_t;

extern logging_methods_t logging_methods;

#endif // LOGGING_H
