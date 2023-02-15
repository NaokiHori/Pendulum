.. _numerical_treatment:

###################
Numerical treatment
###################

.. note::

   In this section, superscripts denote exponents or the time step, while the subscripts are used for the other purposes (identifying each particle or representing tensors).

As derived in :ref:`the governing equations <governing_equations>`, the equations for particle :math:`i` are given by

.. math::

   \frac{d \theta_i}{dt}
   =
   \omega_i,

and

.. math::

   & +
   \sum_{j = i}^{N - 1}
   \sum_{k = 0}^{j  }
   \frac{d \omega_k}{dt}
   \cos \left( \theta_i - \theta_k \right)
   \\
   & +
   \sum_{j = i}^{N - 1}
   \sum_{k = 0}^{j  }
   \left( \omega_k \right)^2
   \sin \left( \theta_i - \theta_k \right)
   \\
   & +
   g \sin \theta_i \left( N - i \right)
   =
   0.

For notational simplicity, I define

.. math::

   c_{i,k}
   & \equiv
   \cos \left( \theta_i - \theta_k \right), \\
   s_{i,k}
   & \equiv
   \sin \left( \theta_i - \theta_k \right)

to write

.. math::

   \frac{d \theta_i}{dt}
   =
   \omega_i,

and

.. math::

   \sum_{j = i}^{N - 1}
   \sum_{k = 0}^{j  }
   \frac{d \omega_k}{dt}
   c_{i,k}
   +
   \sum_{j = i}^{N - 1}
   \sum_{k = 0}^{j  }
   \left( \omega_k \right)^2
   s_{i,k}
   +
   g \sin \theta_i \left( N - i \right)
   =
   0.

*************
Linear system
*************

The above equation can be written as a linear system whose size is :math:`N`:

.. math::

   \newcommand{\elem}[4]{{#1}_{#2,#3} \left( #4 \right)}
   & +
   \begin{bmatrix}
      \elem{c}{  0}{0}{N  } & \elem{c}{  0}{1}{N-1} & \cdots & \elem{c}{  0}{N-1}{1  } \\
      \elem{c}{  1}{0}{N-1} & \elem{c}{  1}{1}{N-1} & \cdots & \elem{c}{  1}{N-1}{1  } \\
      \vdots                & \vdots                & \ddots & \vdots                  \\
      \elem{c}{N-1}{0}{1  } & \elem{c}{N-1}{1}{1  } & \cdots & \elem{c}{N-1}{N-1}{1  }
   \end{bmatrix}
   \begin{bmatrix}
      \frac{d \omega_{  0}}{dt} \\
      \frac{d \omega_{  1}}{dt} \\
      \vdots                    \\
      \frac{d \omega_{N-1}}{dt} \\
   \end{bmatrix} \\
   & = \\
   & -
   \begin{bmatrix}
      \elem{s}{  0}{0}{N  } & \elem{s}{  0}{1}{N-1} & \cdots & \elem{s}{  0}{N-1}{1  } \\
      \elem{s}{  1}{0}{N-1} & \elem{s}{  1}{1}{N-1} & \cdots & \elem{s}{  1}{N-1}{1  } \\
      \vdots                & \vdots                & \ddots & \vdots                  \\
      \elem{s}{N-1}{0}{1  } & \elem{s}{N-1}{1}{1  } & \cdots & \elem{s}{N-1}{N-1}{1  }
   \end{bmatrix}
   \begin{bmatrix}
      \left( \omega_{  0} \right)^2 \\
      \left( \omega_{  1} \right)^2 \\
      \vdots                        \\
      \left( \omega_{N-1} \right)^2 \\
   \end{bmatrix} \\
   & -
   \begin{bmatrix}
      g \sin \theta_{  0} \left( N   \right) \\
      g \sin \theta_{  1} \left( N-1 \right) \\
      \vdots                                         \\
      g \sin \theta_{N-1} \left( 1   \right) \\
   \end{bmatrix},

or equivalently

.. math::

   \left[ N - \max \left( i, j \right) \right] c_{i,j} \frac{d \omega_j}{dt}
   =
   -
   \left[ N - \max \left( i, j \right) \right] s_{i,j} \left( \omega_j \right)^2
   -
   \left( N - i \right) g \sin \theta_i.

Since the system is frictionless, total energy

.. math::

   E
   \equiv
   T
   +
   U
   =
   \sum_{j = 0}^{N - 1} \frac{1}{2} \left[
      \left( \sum_{k = 0}^{j} \omega_k \cos \theta_k \right)^2
      +
      \left( \sum_{k = 0}^{j} \omega_k \sin \theta_k \right)^2
   \right]
   -
   g \sum_{j = 0}^{N-1} \sum_{k = 0}^{j} \cos \theta_k

is conserved in theory:

.. math::

   \frac{dE}{dt}
   =
   0.

Since the equation is non-linear, it is non-trivial to construct a numerical scheme to satisfy this relation.

However, in order to mimic this property as much as possible, I adopt the Crank-Nicolson scheme to discretise the system in time:

.. math::

   \frac{
      \theta_i^{n+1}
      -
      \theta_i^{n  }
   }{\Delta t}
   =
   \frac{
      \omega_i^{n+1}
      +
      \omega_i^{n  }
   }{2},

.. math::

   \frac{1}{2} \left(
      C_{i,j}^{n+1}
      +
      C_{i,j}^{n  }
   \right)
   \frac{
      \omega_j^{n+1}
      -
      \omega_j^{n  }
   }{\Delta t}
   & =
   -
   \frac{1}{2} \left(
      S_{i,j}^{n+1}
      +
      S_{i,j}^{n  }
   \right)
   \cdot
   \frac{1}{2} \left[
      \left( \omega_j^{n+1} \right)^2
      +
      \left( \omega_j^{n  } \right)^2
   \right] \\
   & -
   \frac{N - i}{2} \left(
      g \sin \theta_i^{n+1}
      +
      g \sin \theta_i^{n  }
   \right),

where

.. math::

   C_{i,j}^*
   \equiv
   \left[ N - \max \left( i, j \right) \right] c_{i,j}^*
   =
   \left[ N - \max \left( i, j \right) \right] \cos \left( \theta_i^* - \theta_j^* \right)

and

.. math::

   S_{i,j}^*
   \equiv
   \left[ N - \max \left( i, j \right) \right] s_{i,j}^*
   =
   \left[ N - \max \left( i, j \right) \right] \sin \left( \theta_i^* - \theta_j^* \right),

which is implemented in `src/pendulum/update.c <https://github.com/NaokiHori/Pendulum/blob/main/src/pendulum/update.c>`_.

