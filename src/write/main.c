#include <stdio.h>
#include <string.h>
#include <stdbool.h>
#include "common.h"
#include "write.h"
#include "internal.h"


static void output_mass_info(const double time, const pendulum_t *pendulum){
  // output xy positions for each mass
  const size_t nitems = pendulum_methods.get_nitems(pendulum);
  const char prefix[] = {"output/mass"};
  const char suffix[] = {".dat"};
  const int ndigits = 3;
  static bool is_called = false;
  for(size_t n = 0; n < nitems; n++){
    const size_t nchars = strlen(prefix) + strlen(suffix) + ndigits + 1;
    char *filename = common_calloc(nchars, sizeof(char));
    snprintf(filename, nchars, "%s%0*zu%s", prefix, ndigits, n, suffix);
    FILE *fp = NULL;
    if(is_called){
      fp = common_fopen(filename, "a");
    }else{
      fp = common_fopen(filename, "w");
    }
    const double x = pendulum_methods.get_x(pendulum, n);
    const double y = pendulum_methods.get_y(pendulum, n);
    fprintf(fp, "% .7e % .7e % .7e\n", time, x, y);
    common_fclose(fp);
    common_free(filename);
  }
  is_called = true;
}

// constructor
static writer_t * construct(const double rate, const pendulum_t *pendulum){
  const double time = 0.;
  writer_t *writer = common_calloc(1, sizeof(writer_t));
  output_mass_info(time, pendulum);
  writer->rate = rate;
  // assume current time is 0,
  //   then the next event will happen at "rate"
  writer->next = rate;
#if defined(ENABLE_XWINDOW)
  // set up X connection for in-situ visualisation
  initialise_x(writer);
  update_x(time, writer, pendulum);
#endif // ENABLE_XWINDOW
  return writer;
}

// destructor
static void destruct(writer_t *writer){
#if defined(ENABLE_XWINDOW)
  // close X connection
  finalise_x(writer);
#endif // ENABLE_XWINDOW
  common_free(writer);
}

// getter
static double get_next_time(const writer_t *writer){
  return writer->next;
}

// update display, output mass information to files
static void write(const double time, writer_t *writer, const pendulum_t *pendulum){
  output_mass_info(time, pendulum);
#if defined(ENABLE_XWINDOW)
  // render window with new pendulum positions
  update_x(time, writer, pendulum);
#endif // ENABLE_XWINDOW
  writer->next += writer->rate;
}

write_methods_t write_methods = {
  .construct     = construct,
  .destruct      = destruct,
  .get_next_time = get_next_time,
  .write         = write,
};

