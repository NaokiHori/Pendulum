
.. _potential_energy:

################
Potential energy
################

The potential energy of the :math:`\ib`-th object :math:`U_{\ib}` is given by

.. math::

   U_{\ib}
   \equiv
   -
   m g
   \sum_{\ic = 0}^{\ib}
   l \sin \pos_{\ic}.

Thus the total potential energy :math:`U` is

.. math::

   U
   &
   \equiv
   \sum_{\ib = 0}^{N-1}
   U_{\ib}

   &
   =
   -
   m g l
   \sum_{\ib = 0}^{N-1}
   \sum_{\ic = 0}^{\ib}
   \sin{\pos_{\ic}}

   &
   =
   \pene.

**********
Derivative
**********

First,

.. math::

   \tder{}{t} \pder{U}{\vel_{\ia}}
   =
   0

since :math:`U` is not a function of :math:`\vel_{\ia}`.

On the other hand, I have

.. math::

   \pder{U}{\pos_{\ia}}
   &
   =
   -
   m g l
   \sum_{\ib = 0}^{N-1}
   \left( N - \ib \right)
   \pder{}{\pos_{\ia}} \sin{\pos_{\ib}}

   &
   =
   -
   m g l
   \sum_{\ib = 0}^{N-1}
   \left( N - \ib \right)
   \pder{\pos_{\ib}}{\pos_{\ia}}
   \cos{\pos_{\ib}}

   &
   =
   -
   m g l
   \sum_{\ib = 0}^{N-1}
   \left( N - \ib \right)
   \delta_{\ib \ia}
   \cos{\pos_{\ib}}

   &
   =
   -
   m g l
   \left( N - \ia \right)
   \cos \pos_{\ia}.
