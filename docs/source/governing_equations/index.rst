.. _governing_equations:

###################
Governing equations
###################

.. note::

   In this section, superscripts are used to distinguish each particle, whereas subscripts denote tensor components and Einstein notation is applied.

I consider an Euler-Lagrange equation for a pendulum where :math:`N` objects are connected:

.. math::

   \frac{d}{dt} \frac{\partial L}{\partial \omega^i}
   -
   \frac{\partial L}{\partial \theta^i}
   & =
   0, \\
   \frac{d \theta^i}{dt}
   & =
   \omega^i,

where :math:`L` is the Lagrangian of the system, and :math:`i = 0, 1, \cdots, N - 1` are used to describe each particle whose position is given by

.. math::

   x_j^i
   =
   \underline{e}_x^i \sum_{k = 0}^{i} \left( + l^k \sin \theta^k \right)
   +
   \underline{e}_y^i \sum_{k = 0}^{i} \left( - l^k \cos \theta^k \right).

Note that neighbouring particles are connected by rigid rods, i.e. the distances between neighboring particles are fixed.

If I assume that only the kinetic :math:`T` and the potential :math:`U` energies are involved, I can write the Lagrangian as:

.. math::

   L
   \equiv
   T
   -
   U
   =
   \sum_{i = 0}^{N - 1} T^i
   -
   \sum_{i = 0}^{N - 1} U^i.

In the following sections, these two energy contributions are derived separately, which are combined finally to give :math:`L`.

.. toctree::
   :maxdepth: 1

   potential
   kinetic
   lagrangian

