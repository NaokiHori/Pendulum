#include <stdlib.h>
#include <string.h>
#include <math.h>
#include <float.h>
#include "common.h"
#include "pendulum.h"
#include "internal.h"


static void inverse(const size_t nitems, double * restrict arr){
  /*
   * compute inverse matrix using naive Gaussian elimination
   * NOTE: definitely verbose, to be polished
   */
  double * restrict inv = common_calloc(nitems * nitems, sizeof(double));
  // initialise inverse: identity matrix
  for(size_t i = 0; i < nitems; i++){
    for(size_t j = 0; j < nitems; j++){
      if(i == j){
        inv[i * nitems + j] = 1.;
      }else{
        inv[i * nitems + j] = 0.;
      }
    }
  }
  // Gaussian elimination
  for(size_t i = 0; i < nitems; i++){
    double diag = arr[i * nitems + i];
    if(fabs(diag) < DBL_EPSILON){
      printf("singular matrix\n");
      exit(EXIT_FAILURE);
    }
    const double diaginv = 1. / diag;
    for(size_t j = 0; j < nitems; j++){
      arr[i * nitems + j] *= diaginv;
      inv[i * nitems + j] *= diaginv;
    }
    for(size_t j = 0; j < nitems; j++){
      if(i != j){
        const double val = arr[j * nitems + i];
        for(size_t k = 0; k < nitems; k++){
          arr[j * nitems + k] -= arr[i * nitems + k] * val;
          inv[j * nitems + k] -= inv[i * nitems + k] * val;
        }
      }
    }
  }
  memcpy(arr, inv, sizeof(double) * nitems * nitems);
  common_free(inv);
}

int update(pendulum_t * restrict pendulum, double * restrict dt){
  // integrate the equations in time (from t to t+dt)
  //   to update mass position/velocity/accerelation
  const double g_const = pendulum->g_const;
  const size_t nitems = pendulum->nitems;
  double * restrict ang = pendulum->angles;
  double * restrict vel = pendulum->velocities;
  double * restrict lhs      = common_calloc(nitems * nitems, sizeof(double));
  double * restrict rhs      = common_calloc(nitems,          sizeof(double));
  double * restrict dvel_old = common_calloc(nitems,          sizeof(double));
  double * restrict dvel_new = common_calloc(nitems,          sizeof(double));
  // iterate until converged (Crank-Nicolson scheme)
  // return value to report success = 0 (or failure = 1)
  const size_t itermax = 8;
  *dt = fmin(*dt * 2., pendulum->dt_max);
  while(1){
    for(size_t iter = 0; iter < itermax; iter += 1){
      // initialise linear system
      // right-hand-side vector, potential energy contribution
      for(size_t i = 0; i < nitems; i++){
        const double v_old = vel[i];
        const double v_new = v_old + dvel_new[i];
        const double a_old = ang[i];
        const double a_new = a_old + 0.5 * (v_old + v_new) * (*dt);
        rhs[i] = - 0.5 * (nitems - i) * g_const * (
            + sin(a_old)
            + sin(a_new)
        );
      }
      // matrix part, cos and sin
      for(size_t i = 0; i < nitems; i++){
        for(size_t j = 0; j < nitems; j++){
          const double coef = nitems - fmax(i, j);
          const double vi_old = vel[i];
          const double vj_old = vel[j];
          const double vi_new = vi_old + dvel_new[i];
          const double vj_new = vj_old + dvel_new[j];
          const double ai_old = ang[i];
          const double aj_old = ang[j];
          const double ai_new = ai_old + 0.5 * (vi_old + vi_new) * (*dt);
          const double aj_new = aj_old + 0.5 * (vj_old + vj_new) * (*dt);
          // left-hand-side matrix
          lhs[i * nitems + j] = 0.5 * coef * (
              + cos(ai_old - aj_old)
              + cos(ai_new - aj_new)
          );
          // right-hand-side vector, kinetic energy contribution
          rhs[i] -= 0.5 * coef * (
              + sin(ai_old - aj_old)
              + sin(ai_new - aj_new)
          ) * 0.5 * (
              + pow(vj_old, 2.)
              + pow(vj_new, 2.)
          );
        }
      }
      // inserse left-hand-side matrix
      inverse(nitems, lhs);
      // save previous solution
      memcpy(dvel_old, dvel_new, nitems * sizeof(double));
      // compute dvel = lhs^{-1} x rhs
      for(size_t i = 0; i < nitems; i++){
        dvel_new[i] = 0.;
        for(size_t j = 0; j < nitems; j++){
          dvel_new[i] += lhs[i * nitems + j] * rhs[j] * (*dt);
        }
      }
      // convergence check in terms of velocity
      double residual = 0.;
      for(size_t i = 0; i < nitems; i++){
        residual += fabs(dvel_new[i] - dvel_old[i]);
      }
      if(residual < DBL_EPSILON){
        goto success;
      }
    }
    // reset increment
    memset(dvel_new, 0x00, sizeof(double) * nitems);
    // retry with smaller time step size
    (*dt) *= 0.5;
  }
success:
  // converged, update velocities and positions
  for(size_t i = 0; i < nitems; i++){
    const double v_old = vel[i];
    const double v_new = v_old + dvel_new[i];
    const double a_old = ang[i];
    const double a_new = a_old + 0.5 * (v_old + v_new) * (*dt);
    vel[i] = v_new;
    ang[i] = a_new;
  }
  // clean-up heaps
  common_free(lhs);
  common_free(rhs);
  common_free(dvel_new);
  common_free(dvel_old);
  return 0;
}

