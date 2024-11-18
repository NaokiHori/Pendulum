
.. _energy_conservation:

###################
Energy conservation
###################

The conservation of the total energy states that

.. math::

   \total
   \equiv
   \kinetic
   +
   \potential

is constant, where

.. math::

   \kinetic
   &
   =
   \kene,

   \potential
   &
   =
   \pene.

To derive the energy conservation, we consider to differentiate this equation with respect to time.

***************************
Kinetic energy contribution
***************************

.. math::

   \tder{\kinetic}{t}
   &
   =
   \frac{1}{2} m l^2
   \sum_{\ib = 0}^{N - 1}
   \sum_{\ic = 0}^{N - 1}
   \left\{ N - \max \left( \ib, \ic \right) \right\}
   \tder{\vel_{\ib}}{t}
   \vel_{\ic}
   \cos \left( \pos_{\ib} - \pos_{\ic} \right)

   &
   +
   \frac{1}{2} m l^2
   \sum_{\ib = 0}^{N - 1}
   \sum_{\ic = 0}^{N - 1}
   \left\{ N - \max \left( \ib, \ic \right) \right\}
   \vel_{\ib}
   \tder{\vel_{\ic}}{t}
   \cos \left( \pos_{\ib} - \pos_{\ic} \right)

   &
   +
   \frac{1}{2} m l^2
   \sum_{\ib = 0}^{N - 1}
   \sum_{\ic = 0}^{N - 1}
   \left\{ N - \max \left( \ib, \ic \right) \right\}
   \vel_{\ib}
   \vel_{\ic}
   \tder{}{t}
   \cos \left( \pos_{\ib} - \pos_{\ic} \right),

where the first two terms are equal and the sum is

.. math::

   m l^2
   \sum_{\ib = 0}^{N - 1}
   \sum_{\ic = 0}^{N - 1}
   \left\{ N - \max \left( \ib, \ic \right) \right\}
   \tder{\vel_{\ib}}{t}
   \vel_{\ic}
   \cos \left( \pos_{\ib} - \pos_{\ic} \right),

while the last term is

.. math::

   &
   -
   \frac{1}{2} m l^2
   \sum_{\ib = 0}^{N - 1}
   \sum_{\ic = 0}^{N - 1}
   \left\{ N - \max \left( \ib, \ic \right) \right\}
   \vel_{\ib}
   \vel_{\ic}
   \vel_{\ib}
   \sin \left( \pos_{\ib} - \pos_{\ic} \right)

   &
   +
   \frac{1}{2} m l^2
   \sum_{\ib = 0}^{N - 1}
   \sum_{\ic = 0}^{N - 1}
   \left\{ N - \max \left( \ib, \ic \right) \right\}
   \vel_{\ib}
   \vel_{\ic}
   \vel_{\ic}
   \sin \left( \pos_{\ib} - \pos_{\ic} \right)

   =
   &
   -
   m l^2
   \sum_{\ib = 0}^{N - 1}
   \sum_{\ic = 0}^{N - 1}
   \left\{ N - \max \left( \ib, \ic \right) \right\}
   \vel_{\ib}
   \vel_{\ic}
   \vel_{\ib}
   \sin \left( \pos_{\ib} - \pos_{\ic} \right).

As a result,

.. math::

   \tder{\kinetic}{t}
   &
   =
   m l^2
   \sum_{\ib = 0}^{N - 1}
   \sum_{\ic = 0}^{N - 1}
   \left\{ N - \max \left( \ib, \ic \right) \right\}
   \tder{\vel_{\ib}}{t}
   \vel_{\ic}
   \cos \left( \pos_{\ib} - \pos_{\ic} \right)

   &
   -
   m l^2
   \sum_{\ib = 0}^{N - 1}
   \sum_{\ic = 0}^{N - 1}
   \left\{ N - \max \left( \ib, \ic \right) \right\}
   \vel_{\ib}
   \vel_{\ic}
   \vel_{\ib}
   \sin \left( \pos_{\ib} - \pos_{\ic} \right)

   &
   =
   m l^2
   \sum_{\ic = 0}^{N - 1}
   \vel_{\ic}
   \sum_{\ib = 0}^{N - 1}
   \left\{ N - \max \left( \ic, \ib \right) \right\}
   \left\{
      \tder{\vel_{\ib}}{t}
      \cos \left( \pos_{\ic} - \pos_{\ib} \right)
      +
      \vel_{\ib}
      \vel_{\ib}
      \sin \left( \pos_{\ic} - \pos_{\ib} \right)
   \right\}.

*****************************
Potential energy contribution
*****************************

The potential energy contribution is

.. math::

   \tder{\potential}{t}
   &
   =
   -
   m g l
   \sum_{\ib = 0}^{N - 1}
   \left( N - \ib \right)
   \tder{}{t} \left( \sin \pos_{\ib} \right)

   &
   =
   -
   m g l
   \sum_{\ic = 0}^{N - 1}
   \left( N - \ic \right)
   \vel_{\ic}
   \cos \pos_{\ic}.

*******************
Energy conservation
*******************

The temporal derivative of the total energy results in

.. math::

   \tder{\total}{t}
   =
   \sum_{\ia = 0}^{N - 1}
   \vel_{\ia}
   \left\{
      \lag
   \right\},

which is indeed zero since the relation in the wavy brackets is the left-hand-side of :ref:`the Lagrange's equation <lagrange>`.

