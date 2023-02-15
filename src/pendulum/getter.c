#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include "pendulum.h"
#include "internal.h"


static void sanitise_index(const pendulum_t *pendulum, const size_t index){
  const size_t nitems = pendulum->nitems;
  if(index >= nitems){
    printf("%s: out-of-bounds, index: %zu, while nitems: %zu\n", __func__, index, nitems);
    exit(EXIT_FAILURE);
  }
}

size_t get_nitems(const pendulum_t *pendulum){
  return pendulum->nitems;
}

double get_g_const(const pendulum_t *pendulum){
  return pendulum->g_const;
}

double get_angle(const pendulum_t *pendulum, const size_t index){
  sanitise_index(pendulum, index);
  return pendulum->angles[index];
}

double get_velocity(const pendulum_t *pendulum, const size_t index){
  sanitise_index(pendulum, index);
  return pendulum->velocities[index];
}

double get_x(const pendulum_t *pendulum, const size_t index){
  sanitise_index(pendulum, index);
  double x = 0.;
  for(size_t n = 0; n < index + 1; n++){
    const double a = get_angle (pendulum, n);
    x += sin(a);
  }
  return x;
}

double get_y(const pendulum_t *pendulum, const size_t index){
  sanitise_index(pendulum, index);
  double y = 0.;
  for(size_t n = 0; n < index + 1; n++){
    const double a = get_angle (pendulum, n);
    y -= cos(a);
  }
  return y;
}

double get_dxdt(const pendulum_t *pendulum, const size_t index){
  sanitise_index(pendulum, index);
  double dxdt = 0.;
  for(size_t n = 0; n < index + 1; n++){
    const double a = get_angle   (pendulum, n);
    const double v = get_velocity(pendulum, n);
    dxdt += v * cos(a);
  }
  return dxdt;
}

double get_dydt(const pendulum_t *pendulum, const size_t index){
  sanitise_index(pendulum, index);
  double dydt = 0.;
  for(size_t n = 0; n < index + 1; n++){
    const double a = get_angle   (pendulum, n);
    const double v = get_velocity(pendulum, n);
    dydt += v * sin(a);
  }
  return dydt;
}

