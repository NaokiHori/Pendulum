############
Time marcher
############

We adopt the classical Crank-Nicolson scheme to discretize the evolution of the generalized positions:

.. math::

   \dder{\pos_{\ia}}{t}
   =
   \ave{\vel_{\ia}},

while the generalized velocities are evolved using the energy-conserving scheme:

.. math::

   \dlag.

To this end, we need to solve a linear system with respect to :math:`\dif{\vel_{\ib}}`:

.. math::

   \sum_{\ib = 0}^{N - 1}
   A_{\ia \ib}
   \dif{\vel_{\ib}}
   =
   b_{\ia},

where

.. math::

   A_{\ia \ib}
   &
   \equiv
   \left\{
     N
     -
     \max \left( \ia, \ib \right)
   \right\}
   \ave{\cmat{\ia}{\ib}}
   \left(
     \frac{1}{2}
     +
     \frac{1}{2}
     \frac{
        \ave{
           \vel_{\ia}
           \cmat{\ia}{\ib}
        }
     }{
        \ave{\vel_{\ia}}
        \,
        \ave{\cmat{\ia}{\ib}}
     }
   \right),

   b_{\ia}
   &
   \equiv
   \left( N - \ia \right)
   \text{sinc} \frac{\dif{\pos_{\ia}}}{2}
   \cos \ave{\pos_{\ia}}
   \dif{t}

   &
   -
   \sum_{\ib = 0}^{N - 1}
   \left\{ N - \max \left( \ia, \ib \right) \right\}
   \ave{\vel_{\ib}}
   \,
   \ave{\vel_{\ib}}
   \text{sinc} \left(
     \frac{\dif{\pos_{\ia}}}{2}
     -
     \frac{\dif{\pos_{\ib}}}{2}
   \right)
   \sin \left( \ave{\pos_{\ia}} - \ave{\pos_{\ib}} \right)
   \dif{t}.

