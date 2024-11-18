
.. _kinetic_energy:

##############
Kinetic energy
##############

To begin with, we consider the velocity of the :math:`\ia`-th object on the Cartesian coordinate.

The time derivative of the position vector:

.. math::

   \posvec

yields

.. math::

   -
   \vec{e}_x \sum_{\ib = 0}^{\ia} l \vel_{\ib} \sin{ \pos_{\ib} }
   +
   \vec{e}_y \sum_{\ib = 0}^{\ia} l \vel_{\ib} \cos{ \pos_{\ib} },

which is the velocity of the :math:`\ia`-th object.
The kinetic energy of this object is given by

.. math::

   \kinetic_{\ia}
   =
   \frac{1}{2} m
   \left( \sum_{\ib = 0}^{\ia} l \vel_{\ib} \sin{ \pos_{\ib} } \right)^2
   +
   \frac{1}{2} m
   \left( \sum_{\ib = 0}^{\ia} l \vel_{\ib} \cos{ \pos_{\ib} } \right)^2,

and thus the total kinetic energy of the system is

.. math::

   \kinetic
   &
   \equiv
   \sum_{\ia = 0}^{N - 1} \kinetic_{\ia}

   &
   =
   \sum_{\ia = 0}^{N - 1}
   \frac{1}{2} m
   \left( \sum_{\ib = 0}^{\ia} l \vel_{\ib} \sin{ \pos_{\ib} } \right)^2
   +
   \sum_{\ia = 0}^{N - 1}
   \frac{1}{2} m
   \left( \sum_{\ib = 0}^{\ia} l \vel_{\ib} \cos{ \pos_{\ib} } \right)^2

   &
   =
   \sum_{\ia = 0}^{N - 1}
   \frac{1}{2} m
   \sum_{\ib = 0}^{\ia} l \vel_{\ib} \sin{ \pos_{\ib} }
   \sum_{\ic = 0}^{\ia} l \vel_{\ic} \sin{ \pos_{\ic} }
   +
   \sum_{\ia = 0}^{N - 1}
   \frac{1}{2} m
   \sum_{\ib = 0}^{\ia} l \vel_{\ib} \cos{ \pos_{\ib} }
   \sum_{\ic = 0}^{\ia} l \vel_{\ic} \cos{ \pos_{\ic} }

   &
   =
   \frac{1}{2} m l^2
   \sum_{\ia = 0}^{N - 1}
   \sum_{\ib = 0}^{\ia}
   \sum_{\ic = 0}^{\ia}
   \vel_{\ib} \vel_{\ic}
   \cos \left( \pos_{\ib} - \pos_{\ic} \right).

To proceed, we use the identity:

.. math::

   \sum_{\ia = 0}^{N - 1}
   \sum_{\ib = 0}^{\ia}
   \sum_{\ic = 0}^{\ia}
   Q_{\ib \ic}
   =
   \sum_{\ib = 0}^{N - 1}
   \sum_{\ic = 0}^{N - 1}
   \left\{
     N - \max \left( \ib, \ic \right)
   \right\}
   Q_{\ib \ic}.

.. mydetails:: Derivation

   .. include:: summation.rst

As a consequence, we obtain

.. math::

   \kinetic
   =
   \kene.

*************************
Generalized velocity part
*************************

First of all, we consider

.. math::

   \pder{\kinetic}{\vel_{\ia}}
   &
   =
   \frac{1}{2} m l^2
   \sum_{\ib = 0}^{N - 1}
   \sum_{\ic = 0}^{N - 1}
   \left\{ N - \max \left( \ib, \ic \right) \right\}
   \pder{\vel_{\ib}}{\vel_{\ia}}
   \vel_{\ic}
   \cos \left( \pos_{\ib} - \pos_{\ic} \right)

   &
   +
   \frac{1}{2} m l^2
   \sum_{\ib = 0}^{N - 1}
   \sum_{\ic = 0}^{N - 1}
   \left\{ N - \max \left( \ib, \ic \right) \right\}
   \vel_{\ib}
   \pder{\vel_{\ic}}{\vel_{\ia}}
   \cos \left( \pos_{\ib} - \pos_{\ic} \right),

where the two terms are equal by interchanging the indices to obtain

.. math::

   \pder{\kinetic}{\vel_{\ia}}
   &
   =
   m l^2
   \sum_{\ib = 0}^{N - 1}
   \sum_{\ic = 0}^{N - 1}
   \left\{ N - \max \left( \ib, \ic \right) \right\}
   \vel_{\ib}
   \pder{\vel_{\ic}}{\vel_{\ia}}
   \cos \left( \pos_{\ib} - \pos_{\ic} \right)

   &
   =
   m l^2
   \sum_{\ib = 0}^{N - 1}
   \sum_{\ic = 0}^{N - 1}
   \left\{ N - \max \left( \ib, \ic \right) \right\}
   \vel_{\ib}
   \delta_{\ic \ia}
   \cos \left( \pos_{\ib} - \pos_{\ic} \right)

   &
   =
   m l^2
   \sum_{\ib = 0}^{N - 1}
   \left\{ N - \max \left( \ib, \ia \right) \right\}
   \vel_{\ib}
   \cos \left( \pos_{\ib} - \pos_{\ia} \right)

   &
   =
   m l^2
   \sum_{\ib = 0}^{N - 1}
   \left\{ N - \max \left( \ia, \ib \right) \right\}
   \vel_{\ib}
   \cos \left( \pos_{\ia} - \pos_{\ib} \right).

Differentiating this relation with respect to time leads to

.. math::

   \tder{}{t} \pder{\kinetic}{\vel_{\ia}}
   &
   =
   m l^2
   \sum_{\ib = 0}^{N - 1}
   \left\{ N - \max \left( \ia, \ib \right) \right\}
   \tder{}{t}
   \left\{
      \vel_{\ib}
      \cos \left( \pos_{\ia} - \pos_{\ib} \right)
   \right\}

   &
   =
   m l^2
   \sum_{\ib = 0}^{N - 1}
   \left\{ N - \max \left( \ia, \ib \right) \right\}
   \tder{\vel_{\ib}}{t}
   \cos \left( \pos_{\ia} - \pos_{\ib} \right)

   &
   -
   m l^2
   \sum_{\ib = 0}^{N - 1}
   \left\{ N - \max \left( \ia, \ib \right) \right\}
   \vel_{\ib}
   \left( \vel_{\ia} - \vel_{\ib} \right)
   \sin \left( \pos_{\ia} - \pos_{\ib} \right).

***************************
Generalized coordinate part
***************************

.. math::

   \pder{\kinetic}{\pos_{\ia}}
   &
   =
   \frac{1}{2} m l^2
   \sum_{\ib = 0}^{N - 1}
   \sum_{\ic = 0}^{N - 1}
   \left\{ N - \max \left( \ib, \ic \right) \right\}
   \vel_{\ib} \vel_{\ic}
   \pder{}{\pos_{\ia}}
   \cos \left( \pos_{\ib} - \pos_{\ic} \right)

   &
   =
   -
   \frac{1}{2} m l^2
   \sum_{\ib = 0}^{N - 1}
   \sum_{\ic = 0}^{N - 1}
   \left\{ N - \max \left( \ib, \ic \right) \right\}
   \vel_{\ib} \vel_{\ic}
   \left( \pder{\pos_{\ib}}{\pos_{\ia}} - \pder{\pos_{\ic}}{\pos_{\ia}} \right)
   \sin \left( \pos_{\ib} - \pos_{\ic} \right)

   &
   =
   -
   \frac{1}{2} m l^2
   \sum_{\ib = 0}^{N - 1}
   \sum_{\ic = 0}^{N - 1}
   \left\{ N - \max \left( \ib, \ic \right) \right\}
   \vel_{\ib} \vel_{\ic}
   \left( \delta_{\ib \ia} - \delta_{\ic \ia} \right)
   \sin \left( \pos_{\ib} - \pos_{\ic} \right)

   &
   =
   -
   \frac{1}{2} m l^2
   \sum_{\ib = 0}^{N - 1}
   \sum_{\ic = 0}^{N - 1}
   \left\{ N - \max \left( \ib, \ic \right) \right\}
   \vel_{\ib} \vel_{\ic}
   \delta_{\ib \ia}
   \sin \left( \pos_{\ib} - \pos_{\ic} \right)
   +
   \frac{1}{2} m l^2
   \sum_{\ib = 0}^{N - 1}
   \sum_{\ic = 0}^{N - 1}
   \left\{ N - \max \left( \ib, \ic \right) \right\}
   \vel_{\ib} \vel_{\ic}
   \delta_{\ic \ia}
   \sin \left( \pos_{\ib} - \pos_{\ic} \right)

   &
   =
   -
   \frac{1}{2} m l^2
   \sum_{\ic = 0}^{N - 1}
   \left\{ N - \max \left( \ia, \ic \right) \right\}
   \vel_{\ia} \vel_{\ic}
   \sin \left( \pos_{\ia} - \pos_{\ic} \right)
   +
   \frac{1}{2} m l^2
   \sum_{\ib = 0}^{N - 1}
   \left\{ N - \max \left( \ib, \ia \right) \right\}
   \vel_{\ib} \vel_{\ia}
   \sin \left( \pos_{\ib} - \pos_{\ia} \right)

   &
   =
   -
   m l^2
   \sum_{\ib = 0}^{N - 1}
   \left\{ N - \max \left( \ia, \ib \right) \right\}
   \vel_{\ia} \vel_{\ib}
   \sin \left( \pos_{\ia} - \pos_{\ib} \right).

*****
Total
*****

In the Lagrange's equation, the following terms contribute:

.. math::

   \tder{}{t} \pder{\kinetic}{\vel_{\ia}}
   -
   \pder{\kinetic}{\pos_{\ia}}
   &
   =
   m l^2
   \sum_{\ib = 0}^{N - 1}
   \left\{
      N
      -
      \max \left( \ia, \ib \right)
   \right\}
   \tder{\vel_{\ib}}{t}
   \cos \left( \pos_{\ia} - \pos_{\ib} \right)

   &
   +
   m l^2
   \sum_{\ib = 0}^{N - 1}
   \left\{
      N
      -
      \max \left( \ia, \ib \right)
   \right\}
   \vel_{\ib}
   \vel_{\ib}
   \sin \left( \pos_{\ia} - \pos_{\ib} \right).

