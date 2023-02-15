#if !defined(INTERNAL_WRITER_H)
#define INTERNAL_WRITER_H

typedef struct xobjects_t_ xobjects_t;

struct writer_t_ {
  double rate;
  double next;
  xobjects_t *xobjects;
};

#if defined(ENABLE_XWINDOW)
extern void initialise_x(writer_t *writer);
extern void update_x(const double time, writer_t *writer, const pendulum_t *pendulum);
extern void finalise_x(writer_t *writer);
#endif // ENABLE_XWINDOW

#endif // INTERNAL_WRITER_H
