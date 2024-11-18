
.. _potential_energy:

################
Potential energy
################

The potential energy of the :math:`\ib`-th object :math:`\potential_{\ib}` is given by

.. math::

   \potential_{\ib}
   \equiv
   -
   m g
   \sum_{\ic = 0}^{\ib}
   l \sin \pos_{\ic}.

Thus the total potential energy :math:`\potential` is

.. math::

   \potential
   &
   \equiv
   \sum_{\ib = 0}^{N-1}
   \potential_{\ib}

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

   \tder{}{t} \pder{\potential}{\vel_{\ia}}
   =
   0

since :math:`\potential` is not a function of :math:`\vel_{\ia}`.

On the other hand, we have

.. math::

   \pder{\potential}{\pos_{\ia}}
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

