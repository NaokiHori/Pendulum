#if defined(ENABLE_XWINDOW)

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <X11/Xlib.h>
#include "common.h"
#include "write.h"
#include "internal.h"


struct xobjects_t_ {
  // XWindow objects
  Display *display;
  Window window;
  GC gc;
};

void initialise_x(writer_t *writer){
  writer->xobjects = common_calloc(1, sizeof(xobjects_t));
  Display **display = &(writer->xobjects->display);
  Window *window = &(writer->xobjects->window);
  GC *gc = &(writer->xobjects->gc);
  // open display connection
  *display = XOpenDisplay(NULL);
  if(NULL == *display){
    printf("XOpenDisplay error\n");
    exit(EXIT_FAILURE);
  }
  const int screen = DefaultScreen(*display);
  // create window
  *window = XCreateSimpleWindow(
      /* display   */ *display,
      /* window    */ RootWindow(*display, screen),
      /* x         */ 100,
      /* y         */ 100,
      /* width     */ 800,
      /* height    */ 800,
      /* border_w  */ 1,
      /* border    */ BlackPixel(*display, screen),
      /* backgroud */ BlackPixel(*display, screen)
  );
  XStoreName(*display, *window, "Pendulum");
  XMapWindow(*display, *window);
  // graphic context
  *gc = XCreateGC(
      *display,
      *window,
      0,
      NULL
  );
}

void finalise_x(writer_t *writer){
  Display *display = writer->xobjects->display;
  Window   window  = writer->xobjects->window;
  GC       gc      = writer->xobjects->gc;
  XFreeGC(display, gc);
  XDestroyWindow(display, window);
  XCloseDisplay(display);
  common_free(writer->xobjects);
}

void update_x(const double time, writer_t *writer, const pendulum_t *pendulum){
  // mass size
  const size_t nitems = pendulum_methods.get_nitems(pendulum);
  Display *display = writer->xobjects->display;
  Window   window  = writer->xobjects->window;
  GC       gc      = writer->xobjects->gc;
  // allocate memory for drawing objects
  XSegment *lines = common_calloc(nitems, sizeof(XSegment));
  XArc     *arcs  = common_calloc(nitems, sizeof(    XArc));
  // configure positions and sizes nicely
  //   so that the whole system is in the display
  XWindowAttributes attr = {0};
  XGetWindowAttributes(display, window, &attr);
  const unsigned short x      = attr.x;
  const unsigned short y      = attr.y;
  const unsigned short width  = attr.width;
  const unsigned short height = attr.height;
  const unsigned short pend_originx = x + width  / 2;
  const unsigned short pend_originy = y + height / 4;
  // compute amplification factor
  const double sum_length = 1. * nitems + 0.5;
  const unsigned short ref_length
    = width < height
    ? width  / 2
    : height / 2;
  const double factor = ref_length / sum_length;
  const unsigned short radius = (unsigned short)(0.25 * factor);
  // initialise objects to be drawn
  for(size_t n = 0; n < nitems; n++){
    // lines connecting gravity centers
    const double x1 = n == 0 ? 0. : pendulum_methods.get_x(pendulum, n-1);
    const double y1 = n == 0 ? 0. : pendulum_methods.get_y(pendulum, n-1);
    const double x2 =               pendulum_methods.get_x(pendulum, n  );
    const double y2 =               pendulum_methods.get_y(pendulum, n  );
    lines[n].x1 = (unsigned short)(pend_originx + factor * x1);
    lines[n].y1 = (unsigned short)(pend_originy - factor * y1);
    lines[n].x2 = (unsigned short)(pend_originx + factor * x2);
    lines[n].y2 = (unsigned short)(pend_originy - factor * y2);
    // gravity center to left-top corner
    arcs[n].x      = lines[n].x2 - radius;
    arcs[n].y      = lines[n].y2 - radius;
    arcs[n].width  = 2 * radius;
    arcs[n].height = 2 * radius;
    // angle, from 0 to 360 (circle)
    // multiply 64 to convert degree
    arcs[n].angle1 =   0 * 64;
    arcs[n].angle2 = 360 * 64;
  }
  // clear display before drawing
  XClearWindow(display, window);
  // draw lines
  XSetForeground(display, gc, 0xFFFFFF);
  XDrawSegments(display, window, gc, lines, nitems);
  // draw particles
  XSetForeground(display, gc, 0xFF0000);
  XFillArcs(display, window, gc, arcs, nitems);
  /* // write time */
  const char prefix[] = {"time: "};
  const int ndigits = 5;
  const size_t nchars = strlen(prefix) + (size_t)ndigits + 1;
  char *string = common_calloc(nchars, sizeof(char));
  snprintf(string, nchars, "%s% *.1f", prefix, ndigits, time);
  // position
  const int time_x = 10;
  const int time_y = 10;
  XSetForeground(display, gc, 0xFFFFFF);
  XDrawString(display, window, gc, time_x, time_y, string, nchars - 1);
  common_free(string);
  // update display
  XFlush(display);
  // cleaning up is needed since I use classical pointers
  common_free(lines);
  common_free(arcs);
}

#else

struct xobjects_t_ {
};

#endif // ENABLE_XWINDOW
