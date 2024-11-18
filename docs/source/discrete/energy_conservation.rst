###################
Energy conservation
###################

To construct a stable scheme, we consider :ref:`the conservation of energy <energy_conservation>`:

.. math::

   \total
   \equiv
   \kinetic
   +
   \potential
   =
   const.,

where

.. math::

   \kinetic
   &
   =
   \kene,

   \potential
   &
   =
   \pene.

*******************
Kinetic energy part
*******************

.. math::

   \dder{\kinetic}{t}
   &
   =
   \frac{1}{2} m l^2
   \sum_{\ib = 0}^{N - 1}
   \sum_{\ic = 0}^{N - 1}
   \left\{ N - \max \left( \ib, \ic \right) \right\}
   \dder{}{t}
   \left(
      \vel_{\ib}
      \vel_{\ic}
      \cmat{\ib}{\ic}
   \right)

   &
   =
   \frac{1}{2} m l^2
   \sum_{\ib = 0}^{N - 1}
   \sum_{\ic = 0}^{N - 1}
   \left\{ N - \max \left( \ib, \ic \right) \right\}
   \dder{\vel_{\ib}}{t}
   \ave{
      \vel_{\ic}
      \cmat{\ib}{\ic}
   }

   &
   +
   \frac{1}{2} m l^2
   \sum_{\ib = 0}^{N - 1}
   \sum_{\ic = 0}^{N - 1}
   \left\{ N - \max \left( \ib, \ic \right) \right\}
   \ave{\vel_{\ib}}
   \dder{\vel_{\ic}}{t}
   \ave{\cmat{\ib}{\ic}}

   &
   +
   \frac{1}{2} m l^2
   \sum_{\ib = 0}^{N - 1}
   \sum_{\ic = 0}^{N - 1}
   \left\{ N - \max \left( \ib, \ic \right) \right\}
   \ave{\vel_{\ib}}
   \,
   \ave{\vel_{\ic}}
   \dder{}{t}
   \cmat{\ib}{\ic}.

The first two terms yield

.. math::

   m l^2
   \sum_{\ib = 0}^{N - 1}
   \sum_{\ic = 0}^{N - 1}
   \left\{ N - \max \left( \ib, \ic \right) \right\}
   \dder{\vel_{\ib}}{t}
   \left(
      \frac{1}{2}
      \ave{
         \vel_{\ic}
         \cmat{\ib}{\ic}
      }
      +
      \frac{1}{2}
      \ave{\vel_{\ic}}
      \,
      \ave{\cmat{\ib}{\ic}}
   \right),

while the last term leads to

.. math::

   &
   -
   \frac{1}{2} m l^2
   \sum_{\ib = 0}^{N - 1}
   \sum_{\ic = 0}^{N - 1}
   \left\{ N - \max \left( \ib, \ic \right) \right\}
   \ave{\vel_{\ib}}
   \,
   \ave{\vel_{\ic}}
   \left( \ave{\vel_{\ib}} - \ave{\vel_{\ic}} \right)
   \text{sinc} \left(
      \frac{
         \dif{\pos_{\ib}}
      }{2}
      -
      \frac{
         \dif{\pos_{\ic}}
      }{2}
   \right)
   \sin \left( \ave{\pos_{\ib}} - \ave{\pos_{\ic}} \right)

   =
   &
   -
   \frac{1}{2} m l^2
   \sum_{\ib = 0}^{N - 1}
   \sum_{\ic = 0}^{N - 1}
   \left\{ N - \max \left( \ib, \ic \right) \right\}
   \ave{\vel_{\ib}}
   \,
   \ave{\vel_{\ic}}
   \,
   \ave{\vel_{\ib}}
   \text{sinc} \left(
      \frac{
         \dif{\pos_{\ib}}
      }{2}
      -
      \frac{
         \dif{\pos_{\ic}}
      }{2}
   \right)
   \sin \left( \ave{\pos_{\ib}} - \ave{\pos_{\ic}} \right)

   &
   +
   \frac{1}{2} m l^2
   \sum_{\ib = 0}^{N - 1}
   \sum_{\ic = 0}^{N - 1}
   \left\{ N - \max \left( \ib, \ic \right) \right\}
   \ave{\vel_{\ib}}
   \,
   \ave{\vel_{\ic}}
   \,
   \ave{\vel_{\ic}}
   \text{sinc} \left(
      \frac{
         \dif{\pos_{\ib}}
      }{2}
      -
      \frac{
         \dif{\pos_{\ic}}
      }{2}
   \right)
   \sin \left( \ave{\pos_{\ib}} - \ave{\pos_{\ic}} \right)

   =
   &
   -
   m l^2
   \sum_{\ib = 0}^{N - 1}
   \sum_{\ic = 0}^{N - 1}
   \left\{ N - \max \left( \ib, \ic \right) \right\}
   \ave{\vel_{\ib}}
   \,
   \ave{\vel_{\ic}}
   \,
   \ave{\vel_{\ib}}
   \text{sinc} \left(
      \frac{
         \dif{\pos_{\ib}}
      }{2}
      -
      \frac{
         \dif{\pos_{\ic}}
      }{2}
   \right)
   \sin \left( \ave{\pos_{\ib}} - \ave{\pos_{\ic}} \right).

Note that

.. math::

   \dder{\pos_{\ia}}{t}
   =
   \ave{\vel_{\ia}}

is assumed.

As a result, we have

.. math::

   \dder{\kinetic}{t}
   &
   =
   m l^2
   \sum_{\ib = 0}^{N - 1}
   \sum_{\ic = 0}^{N - 1}
   \left\{ N - \max \left( \ib, \ic \right) \right\}
   \dder{\vel_{\ib}}{t}
   \left(
      \frac{1}{2}
      \ave{
         \vel_{\ic}
         \cmat{\ib}{\ic}
      }
      +
      \frac{1}{2}
      \ave{\vel_{\ic}}
      \,
      \ave{\cmat{\ib}{\ic}}
   \right)

   &
   -
   m l^2
   \sum_{\ib = 0}^{N - 1}
   \sum_{\ic = 0}^{N - 1}
   \left\{ N - \max \left( \ib, \ic \right) \right\}
   \ave{\vel_{\ib}}
   \,
   \ave{\vel_{\ic}}
   \,
   \ave{\vel_{\ib}}
   \text{sinc} \left(
      \frac{
         \dif{\pos_{\ib}}
         -
         \dif{\pos_{\ic}}
      }{2}
   \right)
   \sin \left( \ave{\pos_{\ib}} - \ave{\pos_{\ic}} \right)

   &
   =
   m l^2
   \sum_{\ib = 0}^{N - 1}
   \sum_{\ic = 0}^{N - 1}
   \left\{ N - \max \left( \ib, \ic \right) \right\}
   \dder{\vel_{\ib}}{t}
   \left(
      \frac{1}{2}
      \ave{
         \vel_{\ic}
         \cmat{\ic}{\ib}
      }
      +
      \frac{1}{2}
      \ave{\vel_{\ic}}
      \,
      \ave{\cmat{\ic}{\ib}}
   \right)

   &
   +
   m l^2
   \sum_{\ib = 0}^{N - 1}
   \sum_{\ic = 0}^{N - 1}
   \left\{ N - \max \left( \ib, \ic \right) \right\}
   \ave{\vel_{\ic}}
   \,
   \ave{\vel_{\ib}}
   \,
   \ave{\vel_{\ib}}
   \text{sinc} \left(
      \frac{
         \dif{\pos_{\ic}}
      }{2}
      -
      \frac{
         \dif{\pos_{\ib}}
      }{2}
   \right)
   \sin \left( \ave{\pos_{\ic}} - \ave{\pos_{\ib}} \right).

*********************
Potential energy part
*********************

.. math::

   \dder{\potential}{t}
   &
   =
   -
   m g l
   \sum_{\ic = 0}^{N - 1}
   \left( N - \ic \right)
   \dder{}{t}
   \sin \pos_{\ic}

   &
   =
   -
   m g l
   \sum_{\ic = 0}^{N - 1}
   \left( N - \ic \right)
   \ave{\vel_{\ic}}
   \text{sinc} \frac{\dif{\pos_{\ic}}}{2}
   \cos \ave{\pos_{\ic}}.

************
Total energy
************

The change of the total energy results in

.. math::

   \dder{\total}{t}
   &
   =
   m l^2
   \sum_{\ib = 0}^{N - 1}
   \sum_{\ic = 0}^{N - 1}
   \left\{ N - \max \left( \ib, \ic \right) \right\}
   \ave{\vel_{\ic}}
   \dder{\vel_{\ib}}{t}
   \ave{\cmat{\ic}{\ib}}
   \left(
      \frac{1}{2}
      +
      \frac{1}{2}
      \frac{
         \ave{
            \vel_{\ic}
            \cmat{\ic}{\ib}
         }
      }{
         \ave{\vel_{\ic}}
         \,
         \ave{\cmat{\ic}{\ib}}
      }
   \right)

   &
   +
   m l^2
   \sum_{\ib = 0}^{N - 1}
   \sum_{\ic = 0}^{N - 1}
   \left\{ N - \max \left( \ib, \ic \right) \right\}
   \ave{\vel_{\ic}}
   \,
   \ave{\vel_{\ib}}
   \,
   \ave{\vel_{\ib}}
   \text{sinc} \left(
      \frac{
         \dif{\pos_{\ic}}
      }{2}
      -
      \frac{
         \dif{\pos_{\ib}}
      }{2}
   \right)
   \sin \left( \ave{\pos_{\ic}} - \ave{\pos_{\ib}} \right)

   &
   -
   m g l
   \sum_{\ic = 0}^{N - 1}
   \left( N - \ic \right)
   \ave{\vel_{\ic}}
   \text{sinc} \frac{\dif{\pos_{\ic}}}{2}
   \cos \ave{\pos_{\ic}}

   &
   =
   0.

Factoring :math:`\ave{\vel_{\ic}}` yields

.. math::

   \dder{\total}{t}
   =
   \sum_{\ic = 0}^{N - 1}
   \ave{\vel_{\ic}}
   Q_{\ic}
   =
   0,

where

.. math::

   Q_{\ic}
   &
   \equiv
   m l^2
   \sum_{\ib = 0}^{N - 1}
   \left\{ N - \max \left( \ib, \ic \right) \right\}
   \dder{\vel_{\ib}}{t}
   \ave{\cmat{\ic}{\ib}}
   \left(
      \frac{1}{2}
      +
      \frac{1}{2}
      \frac{
         \ave{
            \vel_{\ic}
            \cmat{\ic}{\ib}
         }
      }{
         \ave{\vel_{\ic}}
         \,
         \ave{\cmat{\ic}{\ib}}
      }
   \right)

   &
   +
   m l^2
   \sum_{\ib = 0}^{N - 1}
   \left\{ N - \max \left( \ib, \ic \right) \right\}
   \ave{\vel_{\ib}}
   \,
   \ave{\vel_{\ib}}
   \text{sinc} \left(
      \frac{
         \dif{\pos_{\ic}}
      }{2}
      -
      \frac{
         \dif{\pos_{\ib}}
      }{2}
   \right)
   \sin \left( \ave{\pos_{\ic}} - \ave{\pos_{\ib}} \right)

   &
   -
   m g l
   \left( N - \ic \right)
   \text{sinc} \frac{\dif{\pos_{\ic}}}{2}
   \cos \ave{\pos_{\ic}}.

Requesting :math:`Q_{\ic} \equiv 0_{\ic}` results in the discrete Lagrange's equation to be handled:

.. math::

   \dlag.

