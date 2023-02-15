#include <stdio.h>
#include "pendulum.h"
#include "logging.h"
#include "write.h"


int main(void){
  /* initialise pendulum solver */
#define NITEMS 6
  const double     angles[NITEMS] = {1.5, 1.5, 1.5, 1.5, 1.5, 1.5, };
  const double velocities[NITEMS] = {0.0, 0.0, 0.0, 0.0, 0.0, 0.0, };
  const double g_const = 1.;
  const double dt_max = 1.e-3;
  pendulum_t *pendulum = pendulum_methods.construct(
      NITEMS,
      angles,
      velocities,
      g_const,
      dt_max
  );
  /* initialise writer (output mass positions to files, update X display) */
  const double write_rate = 1.e-1;
  writer_t *writer = write_methods.construct(write_rate, pendulum);
  /* initialise logger, monitor energy etc. */
  const double logging_rate = 1.e-1;
  logger_t *logger = logging_methods.construct(logging_rate, pendulum);
  /* main loop */
  const double timemax = 1.e+2;
  for(double time = 0., dt = dt_max; time <= timemax; ){
    if(0 != pendulum_methods.update(pendulum, &dt)){
      printf("update error\n");
      break;
    }
    time += dt;
    if(time > logging_methods.get_next_time(logger)){
      logging_methods.write(time, dt, logger, pendulum);
    }
    if(time > write_methods.get_next_time(writer)){
      write_methods.write(time, writer, pendulum);
    }
  }
  /* clean-up objects */
  pendulum_methods.destruct(pendulum);
  write_methods.destruct(writer);
  logging_methods.destruct(logger);
  return 0;
}

