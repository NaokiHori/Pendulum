
.. _discretisation:

##############
Discretisation
##############

*************
Normalisation
*************

As derived in :ref:`the governing equation <governing_equation>`, there are :math:`2 N` unknowns

.. math::

   & \vel_{0}, \vel_{1}, \cdots, \vel_{N - 2}, \vel_{N - 1},

   & \pos_{0}, \pos_{1}, \cdots, \pos_{N - 2}, \pos_{N - 1},

and :math:`2 N` equations:

.. math::

   &
   \tder{\pos_{\ia}}{t}
   =
   \vel_{\ia},

   &
   \lag
   =
   0_{\ia}.

In addition to the fact that the equations being :math:`m`-independent, I normalise these equations by the reference length scale :math:`l` and by the reference time scale :math:`\sqrt{l / g}`, giving

.. math::

   &
   \tder{\pos_{\ia}}{t}
   =
   \vel_{\ia},

   &
   \sum_{\ib = 0}^{N - 1}
   \left\{ N - \max \left( \ia, \ib \right) \right\}
   \tder{\vel_{\ib}}{t}
   \cos \left( \pos_{\ia} - \pos_{\ib} \right)
   +
   \sum_{\ib = 0}^{N - 1}
   \left\{ N - \max \left( \ia, \ib \right) \right\}
   \vel_{\ib}
   \vel_{\ib}
   \sin \left( \pos_{\ia} - \pos_{\ib} \right)
   -
   \left( N - \ia \right)
   \cos \pos_{\ia}
   =
   0_{\ia},

which are treated in the following part.

******
Scheme
******

The simplest scheme would be:

.. _explicit_scheme:

.. math::

   &
   \dder{\pos_{\ia}}{t}
   =
   \vel_{\ia}^{n+1},

   &
   \sum_{\ib = 0}^{N - 1}
   \left\{ N - \max \left( \ia, \ib \right) \right\}
   \dder{\vel_{\ib}}{t}
   \cos \left( \pos_{\ia}^n - \pos_{\ib}^n \right)
   =
   \left( N - \ia \right)
   \cos \pos_{\ia}^n
   -
   \sum_{\ib = 0}^{N - 1}
   \left\{ N - \max \left( \ia, \ib \right) \right\}
   \vel_{\ib}^n
   \vel_{\ib}^n
   \sin \left( \pos_{\ia}^n - \pos_{\ib}^n \right),

which requests to solve only one linear system.

This scheme, however, does not satisfy the energy conservation in general (see :ref:`Example <example>`).

The first part is devoted to find a proper scheme which is discretely energy-conserving, which is followed by the concrete numerical treatment.

.. toctree::
   :maxdepth: 1

   prerequisite
   energy_conservation
   time_marcher

