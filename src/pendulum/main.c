#include "pendulum.h"
#include "internal.h"


pendulum_methods_t pendulum_methods = {
  .construct    = construct,
  .destruct     = destruct,
  .get_nitems   = get_nitems,
  .get_g_const  = get_g_const,
  .get_angle    = get_angle,
  .get_velocity = get_velocity,
  .get_x        = get_x,
  .get_y        = get_y,
  .get_dxdt     = get_dxdt,
  .get_dydt     = get_dydt,
  .update       = update,
};

