#include "pendulum.h"
#include "common.h"
#include "internal.h"


void destruct(pendulum_t *pendulum){
  common_free(pendulum->angles);
  common_free(pendulum->velocities);
  common_free(pendulum);
}

