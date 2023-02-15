#if !defined(PENDULUM_H)
#define PENDULUM_H

#include <stddef.h>

typedef struct pendulum_t_ pendulum_t;

typedef struct {
  // constructor
  pendulum_t * (*construct)(
      const size_t nitems,
      const double *angles,
      const double *velocities,
      const double g_const,
      const double dt_max
      );
  // destructor
  void (*destruct)(
      pendulum_t *pendulum
      );
  // getters
  // get number of masses
  size_t (*get_nitems)(
      const pendulum_t *pendulum
      );
  // get gravity acceleration
  double (*get_g_const)(
      const pendulum_t *pendulum
      );
  // get rotation angle of n-th mass
  double (*get_angle)(
      const pendulum_t *pendulum,
      const size_t index
      );
  // get rotation velocity of n-th mass
  double (*get_velocity)(
      const pendulum_t *pendulum,
      const size_t index
      );
  // get x position of n-th mass
  double (*get_x)(
      const pendulum_t *pendulum,
      const size_t index
      );
  // get y position of n-th mass
  double (*get_y)(
      const pendulum_t *pendulum,
      const size_t index
      );
  // get x velocity of n-th mass
  double (*get_dxdt)(
      const pendulum_t *pendulum,
      const size_t index
      );
  // get y velocity of n-th mass
  double (*get_dydt)(
      const pendulum_t *pendulum,
      const size_t index
      );
  // update velocity and position of all masses
  int (*update)(
      pendulum_t *pendulum,
      double *dt
      );
} pendulum_methods_t;

extern pendulum_methods_t pendulum_methods;

#endif // PENDULUM_H
