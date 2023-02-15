#include <string.h>
#include "common.h"
#include "internal.h"


pendulum_t *construct(const size_t nitems, const double *angles, const double *velocities, const double g_const, const double dt_max){
  pendulum_t *pendulum = common_calloc(     1, sizeof(pendulum_t));
  pendulum->angles     = common_calloc(nitems, sizeof(   double));
  pendulum->velocities = common_calloc(nitems, sizeof(   double));
  pendulum->nitems     = nitems;
  pendulum->g_const    = g_const;
  pendulum->dt_max     = dt_max;
  memcpy(    pendulum->angles,     angles, nitems * sizeof(double));
  memcpy(pendulum->velocities, velocities, nitems * sizeof(double));
  return pendulum;
}

