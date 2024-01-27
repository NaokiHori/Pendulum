
.. _example:

#######
Example
#######

The results of the default case with :math:`N = 8` are discussed below.
All objects are horizontally aligned (:math:`\pos_{\ia} = 0`) with a constant angular velocity:

.. math::

   \vel_{\ia}
   =
   \sqrt{\frac{6}{2 N + 1}},

such that a vertically-aligned and stationary state

.. math::

   \pos_{\ia}
   =
   \frac{\pi}{2}

is one (unstable) solution of the system.

To highlight the effect of the scheme, the energy-conserving scheme and :ref:`the explicit scheme <explicit_scheme>` are compared.

************************
Energy-conserving scheme
************************

.. code-block:: console

   cargo run --release

The kinetic energy (:math:`T`, red), the potential energy (:math:`U`, blue), and their sum (:math:`E`, black) are plotted as a function of time:

.. image:: data/energy11.jpg
   :width: 40%

The deviation of the total energy is around the rounding error:

.. image:: data/energy12.jpg
   :width: 40%

***************
Explicit scheme
***************

.. code-block:: console

   cargo run --release --features=explicit

.. image:: data/energy21.jpg
   :width: 40%

The deviation of the total energy is *not* around the rounding error:

.. image:: data/energy22.jpg
   :width: 40%

Although this behaviour is improved when the angle is updated using the Crank-Nicolson scheme:

.. math::

   \dder{\pos_{\ia}}{t}
   =
   \ave{\vel_{\ia}},

a clear decreasing trend is still observable.
With the Euler explicit scheme:

.. math::

   \dder{\pos_{\ia}}{t}
   =
   \vel_{\ia}^{n},

on the other hand, a clear increasing trend is observed.
In short, as long as a fully-explicit scheme is adopted to update the angular velocity, there is not way to conserve the total energy.
A simple remedy is, of course, to make :math:`\delta t` smaller.

