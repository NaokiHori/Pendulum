.. _kinetic_energy:

##############
Kinetic energy
##############

In :ref:`the previous section <potential_energy>`, I derived the total potential energy of the system.
In order to give the total kinetic energy here, I consider the time derivative of the position vector (namely the velocity vector):

.. math::

   u_j^i
   =
   \underline{e}_x^i \sum_{k = 0}^{i} \left( + l^k \omega^k \cos \theta^k \right)
   +
   \underline{e}_y^i \sum_{k = 0}^{i} \left( + l^k \omega^k \sin \theta^k \right).

Thus the kinetic energy of this mass :math:`i` is given by

.. math::

   T^i
   \equiv
   \frac{1}{2} m^i u_j^i u_j^i
   =
   \frac{1}{2} m^i \left[
      \left( \sum_{k = 0}^{i} l^k \omega^k \cos \theta^k \right)^2
      +
      \left( \sum_{k = 0}^{i} l^k \omega^k \sin \theta^k \right)^2
   \right],

and the total kinetic energy of the system results in

.. math::

   T
   =
   \sum_{j = 0}^{N - 1} T^j
   =
   \sum_{j = 0}^{N - 1} \frac{1}{2} m^j \left[
      \left( \sum_{k = 0}^{j} l^k \omega^k \cos \theta^k \right)^2
      +
      \left( \sum_{k = 0}^{j} l^k \omega^k \sin \theta^k \right)^2
   \right].

There are two terms in the Lagrangian that come from the kinetic energy.

#. Time derivative of :math:`\partial T / \partial \omega^i`

   The first sum is divided as

   .. math::

      \sum_{j = 0}^{i - 1} \frac{1}{2} m^j \left(
         \sum_{k = 0}^{j} l^k \omega^k \cos \theta^k
      \right)^2
      +
      \sum_{j = i}^{N - 1} \frac{1}{2} m^j \left(
         \sum_{k = 0}^{j} l^k \omega^k \cos \theta^k
      \right)^2

   and

   .. math::

      \sum_{j = 0}^{i - 1} \frac{1}{2} m^j \left(
         \sum_{k = 0}^{j} l^k \omega^k \sin \theta^k
      \right)^2
      +
      \sum_{j = i}^{N - 1} \frac{1}{2} m^j \left(
         \sum_{k = 0}^{j} l^k \omega^k \sin \theta^k
      \right)^2.

   The first parts disappear because they do not contain :math:`\omega^i`, while the second parts remain.

   The derivative of

   .. math::

      + &
      \frac{1}{2} m^{i  } \left(
         \sum_{k = 0}^{i  } l^k \omega^k \cos \theta^k
      \right)^2 \\
      + &
      \frac{1}{2} m^{i+1} \left(
         \sum_{k = 0}^{i+1} l^k \omega^k \cos \theta^k
      \right)^2 \\
      + &
      \cdots \\
      + &
      \frac{1}{2} m^{N-2} \left(
         \sum_{k = 0}^{N-2} l^k \omega^k \cos \theta^k
      \right)^2 \\
      + &
      \frac{1}{2} m^{N-1} \left(
         \sum_{k = 0}^{N-1} l^k \omega^k \cos \theta^k
      \right)^2

   with respect to :math:`\omega^i` results in

   .. math::

      + &
      m^{i  } l^i \cos \theta^i \left(
         \sum_{k = 0}^{i  } l^k \omega^k \cos \theta^k
      \right) \\
      + &
      m^{i+1} l^i \cos \theta^i \left(
         \sum_{k = 0}^{i+1} l^k \omega^k \cos \theta^k
      \right) \\
      + &
      \cdots \\
      + &
      m^{N-2} l^i \cos \theta^i \left(
         \sum_{k = 0}^{N-2} l^k \omega^k \cos \theta^k
      \right) \\
      + &
      m^{N-1} l^i \cos \theta^i \left(
         \sum_{k = 0}^{N-1} l^k \omega^k \cos \theta^k
      \right).

   The derivative of

   .. math::

      + &
      \frac{1}{2} m^{i  } \left(
         \sum_{k = 0}^{i  } l^k \omega^k \sin \theta^k
      \right)^2 \\
      + &
      \frac{1}{2} m^{i+1} \left(
         \sum_{k = 0}^{i+1} l^k \omega^k \sin \theta^k
      \right)^2 \\
      + &
      \cdots \\
      + &
      \frac{1}{2} m^{N-2} \left(
         \sum_{k = 0}^{N-2} l^k \omega^k \sin \theta^k
      \right)^2 \\
      + &
      \frac{1}{2} m^{N-1} \left(
         \sum_{k = 0}^{N-1} l^k \omega^k \sin \theta^k
      \right)^2

   with respect to :math:`\omega^i` results in

   .. math::

      + &
      m^{i  } l^i \sin \theta^i \left(
         \sum_{k = 0}^{i  } l^k \omega^k \sin \theta^k
      \right) \\
      + &
      m^{i+1} l^i \sin \theta^i \left(
         \sum_{k = 0}^{i+1} l^k \omega^k \sin \theta^k
      \right) \\
      + &
      \cdots \\
      + &
      m^{N-2} l^i \sin \theta^i \left(
         \sum_{k = 0}^{N-2} l^k \omega^k \sin \theta^k
      \right) \\
      + &
      m^{N-1} l^i \sin \theta^i \left(
         \sum_{k = 0}^{N-1} l^k \omega^k \sin \theta^k
      \right).

   Thus I found

   .. math::

      \frac{\partial T}{\partial \omega^i}
      & =
      l^i \cos \theta^i \sum_{j = i}^{N - 1} \left[
         m^{j  } \left(
            \sum_{k = 0}^{j  } l^k \omega^k \cos \theta^k
         \right)
      \right] \\
      & +
      l^i \sin \theta^i \sum_{j = i}^{N - 1} \left[
         m^{j  } \left(
            \sum_{k = 0}^{j  } l^k \omega^k \sin \theta^k
         \right)
      \right].

   Next, I consider the time derivative of this:

   .. math::

      \frac{d}{dt} \frac{\partial T}{\partial \omega^i}.

   Six groups exist:

   .. math::

      & +
      l^i \frac{d}{dt} \left( \cos \theta^i \right) \sum_{j = i}^{N - 1} \left[
         m^{j  } \left(
            \sum_{k = 0}^{j  } l^k \omega^k \cos \theta^k
         \right)
      \right] \\
      & +
      l^i \frac{d}{dt} \left( \sin \theta^i \right) \sum_{j = i}^{N - 1} \left[
         m^{j  } \left(
            \sum_{k = 0}^{j  } l^k \omega^k \sin \theta^k
         \right)
      \right] \\
      & +
      l^i \cos \theta^i \sum_{j = i}^{N - 1} \left[
         m^{j  } \left(
            \sum_{k = 0}^{j  } l^k \frac{d \omega^k}{dt} \cos \theta^k
         \right)
      \right] \\
      & +
      l^i \sin \theta^i \sum_{j = i}^{N - 1} \left[
         m^{j  } \left(
            \sum_{k = 0}^{j  } l^k \frac{d \omega^k}{dt} \sin \theta^k
         \right)
      \right] \\
      & +
      l^i \cos \theta^i \sum_{j = i}^{N - 1} \left[
         m^{j  } \left(
            \sum_{k = 0}^{j  } l^k \omega^k \frac{d}{dt} \left( \cos \theta^k \right)
         \right)
      \right] \\
      & +
      l^i \sin \theta^i \sum_{j = i}^{N - 1} \left[
         m^{j  } \left(
            \sum_{k = 0}^{j  } l^k \omega^k \frac{d}{dt} \left( \sin \theta^k \right)
         \right)
      \right].

   First and second:

   .. math::

      & -
      l^i \omega^i \sin \theta^i \sum_{j = i}^{N - 1} \left[
         m^{j  } \left(
            \sum_{k = 0}^{j  } l^k \omega^k \cos \theta^k
         \right)
      \right] \\
      & +
      l^i \omega^i \cos \theta^i \sum_{j = i}^{N - 1} \left[
         m^{j  } \left(
            \sum_{k = 0}^{j  } l^k \omega^k \sin \theta^k
         \right)
      \right].

   The pre-factors can be moved into the summations, giving

   .. math::

      \sum_{j = i}^{N - 1} \left[
         m^{j  } \left\{
            \sum_{k = 0}^{j  }
            \left(
               - l^i \omega^i \sin \theta^i l^k \omega^k \cos \theta^k
               + l^i \omega^i \cos \theta^i l^k \omega^k \sin \theta^k
            \right)
         \right\}
      \right].

   By using the identity

   .. math::

      - \sin \theta^i \cos \theta^k
      + \cos \theta^i \sin \theta^k
      \equiv
      - \sin \left( \theta^i - \theta^k \right),

   I have

   .. math::

      -
      \sum_{j = i}^{N - 1} \left[
         m^{j  }
         \sum_{k = 0}^{j  }
         l^i l^k \omega^i \omega^k
         \sin \left( \theta^i - \theta^k \right)
      \right].

   Third and fourth:

   .. math::

      & +
      l^i \cos \theta^i \sum_{j = i}^{N - 1} \left[
         m^{j  } \left(
            \sum_{k = 0}^{j  } l^k \frac{d \omega^k}{dt} \cos \theta^k
         \right)
      \right] \\
      & +
      l^i \sin \theta^i \sum_{j = i}^{N - 1} \left[
         m^{j  } \left(
            \sum_{k = 0}^{j  } l^k \frac{d \omega^k}{dt} \sin \theta^k
         \right)
      \right]

   yields

   .. math::

      \sum_{j = i}^{N - 1} \left[
         m^{j  }
         \sum_{k = 0}^{j  }
         l^i l^k \frac{d \omega^k}{dt}
         \cos \left( \theta^i - \theta^k \right)
      \right]

   where another identity

   .. math::

      \cos \theta^i \cos \theta^k
      +
      \sin \theta^i \sin \theta^k
      \equiv
      \cos \left( \theta^i - \theta^k \right)

   was adopted.

   Fifth and sixth:

   .. math::

      & -
      l^i \cos \theta^i \sum_{j = i}^{N - 1} \left[
         m^{j  } \left(
            \sum_{k = 0}^{j  } l^k \omega^k \omega^k \sin \theta^k
         \right)
      \right] \\
      & +
      l^i \sin \theta^i \sum_{j = i}^{N - 1} \left[
         m^{j  } \left(
            \sum_{k = 0}^{j  } l^k \omega^k \omega^k \cos \theta^k
         \right)
      \right]

   yields

   .. math::

      \sum_{j = i}^{N - 1} \left[
         m^{j  }
         \sum_{k = 0}^{j  }
         l^i l^k \omega^k \omega^k
         \sin \left( \theta^i - \theta^k \right)
      \right].

   Finally I have

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
      \right].

#. :math:`\partial T / \partial \theta^i`

   For convenience, the total kinetic energy is shown here again:

   .. math::

      T
      =
      \sum_{j = 0}^{N - 1} T^j
      =
      \sum_{j = 0}^{N - 1} \frac{1}{2} m^j \left[
         \left( \sum_{k = 0}^{j} l^k \omega^k \cos \theta^k \right)^2
         +
         \left( \sum_{k = 0}^{j} l^k \omega^k \sin \theta^k \right)^2
      \right].

   By splitting this into two segments

   .. math::

      + &
      \sum_{j = 0}^{i - 1} \frac{1}{2} m^j \left[
         \left( \sum_{k = 0}^{j} l^k \omega^k \cos \theta^k \right)^2
         +
         \left( \sum_{k = 0}^{j} l^k \omega^k \sin \theta^k \right)^2
      \right] \\
      + &
      \sum_{j = i}^{N - 1} \frac{1}{2} m^j \left[
         \left( \sum_{k = 0}^{j} l^k \omega^k \cos \theta^k \right)^2
         +
         \left( \sum_{k = 0}^{j} l^k \omega^k \sin \theta^k \right)^2
      \right],

   I find the first term vanishes, while the second term yields

   .. math::

      \frac{\partial T}{\partial \theta^i}
      =
      & -
      l^i \omega^i \sin \theta^i
      \sum_{j = i}^{N - 1} \left[
        m^j
        \sum_{k = 0}^{j} l^k \omega^k \cos \theta^k
      \right] \\
      & +
      l^i \omega^i \cos \theta^i
      \sum_{j = i}^{N - 1} \left[
        m^j
        \sum_{k = 0}^{j} l^k \omega^k \sin \theta^k
      \right],

   or equivalently

   .. math::

      \frac{\partial T}{\partial \theta^i}
      =
      - \sum_{j = i}^{N - 1} \left[
        m^j
        \sum_{k = 0}^{j}
        l^i l^k \omega^i \omega^k
        \sin \left( \theta^i - \theta^k \right)
      \right].

In :ref:`the next section <lagrangian>`, Lagrangian of the system will be discussed.

