.. _potential_energy:

################
Potential energy
################

If gravity acts downwards in the :math:`y`-direction with a gravitational acceleration :math:`g`, the potential energy of this mass :math:`U^i` is given by:

.. math::

   U^i
   \equiv
   m^i
   g
   \left( x_j^i \delta_{jy} \right)
   =
   m^i g \sum_{k = 0}^{i} \left( - l^k \cos \theta^k \right),

giving the total potential energy:

.. math::

   U
   \equiv
   \sum_{j = 0}^{N-1} U^j
   =
   \sum_{j = 0}^{N-1} m^j g \sum_{k = 0}^{j} \left( - l^k \cos \theta^k \right).

There are two terms in the Lagrangian that come from the potential energy.

#. Time derivative of :math:`\partial U / \partial \omega^i`

   Since

   .. math::

      \frac{\partial U}{\partial \omega^i}
      =
      0,

   I have

   .. math::

      \frac{d}{dt} \frac{\partial U}{\partial \omega^i}
      =
      0.

#. :math:`\partial U / \partial \theta^i`

   I consider to split the first summation as

   .. math::

      \sum_{j = 0}^{i-1} m^j g \sum_{k = 0}^{j} \left( - l^k \cos \theta^k \right)
      +
      \sum_{j = i}^{N-1} m^j g \sum_{k = 0}^{j} \left( - l^k \cos \theta^k \right).

   The first part disappears and only the second part

   .. math::

      \sum_{j = i}^{N-1} m^j g \sum_{k = 0}^{j} \left( - l^k \cos \theta^k \right)

   remains since the first part does not include :math:`\theta^i`.
   In addition, I have

   .. math::

      + & m^{i  } g \sum_{k = 0}^{i  } \left( - l^k \cos \theta^k \right) \\
      + & m^{i+1} g \sum_{k = 0}^{i+1} \left( - l^k \cos \theta^k \right) \\
      + & \cdots \\
      + & m^{N-2} g \sum_{k = 0}^{N-2} \left( - l^k \cos \theta^k \right) \\
      + & m^{N-1} g \sum_{k = 0}^{N-1} \left( - l^k \cos \theta^k \right).

   The derivative of these terms with respect to :math:`\theta^i` gives

   .. math::

      + & m^{i  } g \left( + l^i \sin \theta^i \right) \\
      + & m^{i+1} g \left( + l^i \sin \theta^i \right) \\
      + & \cdots \\
      + & m^{N-2} g \left( + l^i \sin \theta^i \right) \\
      + & m^{N-1} g \left( + l^i \sin \theta^i \right),

   or

   .. math::

      g l^i \sin \theta^i \sum_{j = i}^{N - 1} m^j.

The total kinetic energy will be discussed in :ref:`the next section <kinetic_energy>`.

