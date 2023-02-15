.. _lagrangian:

##########
Lagrangian
##########

:ref:`The potential energy contributions derived in the previous section <potential_energy>` are

.. math::

   \frac{d}{dt} \frac{\partial U}{\partial \omega^i}
   =
   0

and

.. math::

   \frac{\partial U}{\partial \theta^i}
   =
   g l^i \sin \theta^i \sum_{j = i}^{N - 1} m^j,

while :ref:`the ones of the kinetic energy <kinetic_energy>` are

.. math::

   \frac{d}{dt} \frac{\partial T}{\partial \omega^i}
   =
   & -
   \sum_{j = i}^{N - 1} \left[
      m^{j  }
      \sum_{k = 0}^{j  }
      l^i l^k \omega^i \omega^k
      \sin \left( \theta^i - \theta^k \right)
   \right] \\
   & +
   \sum_{j = i}^{N - 1} \left[
      m^{j  }
      \sum_{k = 0}^{j  }
      l^i l^k \frac{d \omega^k}{dt}
      \cos \left( \theta^i - \theta^k \right)
   \right] \\
   & +
   \sum_{j = i}^{N - 1} \left[
      m^{j  }
      \sum_{k = 0}^{j  }
      l^i l^k \omega^k \omega^k
      \sin \left( \theta^i - \theta^k \right)
   \right]

and

.. math::

   \frac{\partial T}{\partial \theta^i}
   =
   - \sum_{j = i}^{N - 1} \left[
     m^j
     \sum_{k = 0}^{j}
     l^i l^k \omega^i \omega^k
     \sin \left( \theta^i - \theta^k \right)
   \right].

Since :math:`L \equiv T - U`, I have

.. math::

   \frac{d}{dt} \frac{\partial L}{\partial \omega^i}
   -
   \frac{\partial L}{\partial \theta^i}
   =
   \frac{d}{dt} \frac{\partial T}{\partial \omega^i}
   -
   \frac{\partial T}{\partial \theta^i}
   -
   \frac{d}{dt} \frac{\partial U}{\partial \omega^i}
   +
   \frac{\partial U}{\partial \theta^i}
   =
   0.

Thus, I obtain

.. math::

   & +
   \sum_{j = i}^{N - 1} \left[
      m^{j  }
      \sum_{k = 0}^{j  }
      l^i l^k \frac{d \omega^k}{dt}
      \cos \left( \theta^i - \theta^k \right)
   \right] \\
   & +
   \sum_{j = i}^{N - 1} \left[
      m^{j  }
      \sum_{k = 0}^{j  }
      l^i l^k \left( \omega^k \right)^2
      \sin \left( \theta^i - \theta^k \right)
   \right] \\
   & +
   g l^i \sin \theta^i \sum_{j = i}^{N - 1} m^j
   =
   0,

which is the equation to be solved numerically.

For the time being, for simplicity, I fix :math:`m^i` and :math:`l^i` to :math:`1`, yielding

.. math::

   & +
   \sum_{j = i}^{N - 1}
   \sum_{k = 0}^{j  }
   \frac{d \omega^k}{dt}
   \cos \left( \theta^i - \theta^k \right)
   \\
   & +
   \sum_{j = i}^{N - 1}
   \sum_{k = 0}^{j  }
   \left( \omega^k \right)^2
   \sin \left( \theta^i - \theta^k \right)
   \\
   & +
   g \sin \theta^i \left( N - i \right)
   =
   0.

How I treat this equation numerically will be discussed in :ref:`the next chapter <numerical_treatment>`.

