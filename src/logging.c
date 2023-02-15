#include <stdio.h>
#include <stdbool.h>
#include <math.h>
#include "common.h"
#include "pendulum.h"
#include "logging.h"


struct logger_t_ {
  double energies_0[2];
  double rate;
  double next;
};

static void check_energy(const pendulum_t *pendulum, double *energies){
  const size_t nitems  = pendulum_methods.get_nitems(pendulum);
  const double g_const = pendulum_methods.get_g_const(pendulum);
  energies[0] = 0.;
  energies[1] = 0.;
  for(size_t n = 0; n < nitems; n++){
    const double y    = pendulum_methods.get_y   (pendulum, n);
    const double dxdt = pendulum_methods.get_dxdt(pendulum, n);
    const double dydt = pendulum_methods.get_dydt(pendulum, n);
    energies[0] += y;
    energies[1] +=
      + 0.5 * g_const * pow(dxdt, 2.)
      + 0.5 * g_const * pow(dydt, 2.);
  }
}

static void output_energy(const double time, const double *energies, const double *energies_0){
  const char filename[] = {"output/energy.dat"};
  static bool is_called = false;
  FILE *fp = NULL;
  if(is_called){
    fp = common_fopen(filename, "a");
  }else{
    fp = common_fopen(filename, "w");
  }
  const double de0 = energies[0] - energies_0[0];
  const double de1 = energies[1] - energies_0[1];
  fprintf(fp, "% .7e % .7e % .7e % .7e\n", time, de0, de1, de0 + de1);
  common_fclose(fp);
  is_called = true;
}

// constructor
static logger_t * construct(const double rate, const pendulum_t *pendulum){
  logger_t *logger = common_calloc(1, sizeof(logger_t));
  // set initial energy
  double *energies_0 = logger->energies_0;
  check_energy(pendulum, energies_0);
  output_energy(0., energies_0, energies_0);
  logger->rate = rate;
  // assume current time is 0,
  //   then the next event will happen at "rate"
  logger->next = rate;
  return logger;
}

// destructor
static void destruct(logger_t *logger){
  common_free(logger);
}

// getter
static double get_next_time(const logger_t *logger){
  return logger->next;
}

void write(const double time, const double dt, logger_t *logger, const pendulum_t *pendulum){
  double energies[2] = {0.};
  check_energy(pendulum, energies);
  output_energy(time, energies, logger->energies_0);
  printf("time: % .2e dt: % .2e\n", time, dt);
  logger->next += logger->rate;
}

logging_methods_t logging_methods = {
  .construct     = construct,
  .destruct      = destruct,
  .get_next_time = get_next_time,
  .write         = write,
};

