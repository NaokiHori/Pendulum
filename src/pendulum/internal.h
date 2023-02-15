#if !defined(INTERNAL_PENDULUM_H)
#define INTERNAL_PENDULUM_H

#include <stdlib.h>
#include "pendulum.h"

struct pendulum_t_ {
  double dt_max;
  size_t nitems;
  double g_const;
  double *angles;
  double *velocities;
};

extern pendulum_t *construct(const size_t nitems, const double *angles, const double *velocities, const double g_const, const double dt_max);
extern void destruct(pendulum_t *pendulum);

extern int update(pendulum_t *pendulum, double *dt);

extern size_t get_nitems  (const pendulum_t *pendulum);
extern double get_g_const (const pendulum_t *pendulum);
extern double get_angle   (const pendulum_t *pendulum, const size_t index);
extern double get_velocity(const pendulum_t *pendulum, const size_t index);
extern double get_x       (const pendulum_t *pendulum, const size_t index);
extern double get_y       (const pendulum_t *pendulum, const size_t index);
extern double get_dxdt    (const pendulum_t *pendulum, const size_t index);
extern double get_dydt    (const pendulum_t *pendulum, const size_t index);

#endif // INTERNAL_PENDULUM_H
