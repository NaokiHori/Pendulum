
.. _governing_equation:

##################
Governing equation
##################

We consider a :math:`N`-body pendulum, where :math:`N` objects are rigidly connected with massless rods and the gravitational acceleration :math:`g` works in the :math:`y` direction.
For the sake of convenience, we assume that all objects have the same mass :math:`m` and all rods have the same length :math:`l`.

Equations which describe the motion of this system are derived in this section.
Hereafter the objects are distinguished by subscripts (:math:`\ia, \ib, \ic`), which take :math:`0, 1, \cdots, N - 1`.

On the Cartesian coordinate, the position of the :math:`\ia`-th object is given by

.. math::

   \posvec,

where :math:`\pos_{\ia} = \pos_{\ia} \left( t \right)` is the rotational angle measured from the :math:`x` axis, and :math:`\vel_{\ia} = \vel_{\ia} \left( t \right)` is its time derivative:

.. math::

   \vel_{\ia}
   =
   \tder{\pos_{\ia}}{t}.

Their motions are governed by the Euler-Lagrange equation:

.. math::

   \tder{}{t} \pder{L}{\vel_{\ia}}
   -
   \pder{L}{\pos_{\ia}}
   =
   0_{\ia} \,\, \left( N \text{-dimensional zero vector} \right),

where :math:`L = L \left( \pos_{\ia}, \vel_{\ia} \right)` is the Lagrangian of the system with the basic variables :math:`\pos_{\ia}`: generalized coordinate and :math:`\vel_{\ia}`: its time derivative.

In this project, we consider the contributions of the kinetic energy :math:`\kinetic = \kinetic \left( \pos_{\ia}, \vel_{\ia} \right)` and the potential energy :math:`\potential = \potential \left( \pos_{\ia} \right)` to describe the Newtonian mechanics:

.. math::

   L
   \equiv
   \kinetic
   -
   \potential
   =
   \sum_{\ia = 0}^{N - 1} \kinetic_{\ia}
   -
   \sum_{\ia = 0}^{N - 1} \potential_{\ia}.

Also, by assuming that the system is frictionless, the total energy:

.. math::

   \total \left( \pos_{\ia}, \vel_{\ia} \right)
   \equiv
   \kinetic
   +
   \potential
   =
   \sum_{\ia = 0}^{N - 1} \kinetic_{\ia}
   +
   \sum_{\ia = 0}^{N - 1} \potential_{\ia}

is conserved:

.. math::

   \tder{\total}{t}
   =
   0.

The following pages are devoted to derive these relations for the :math:`N`-body pendulums.

.. toctree::
   :maxdepth: 1

   kinetic
   potential
   lagrange
   energy_conservation

