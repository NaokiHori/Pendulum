####################
Symbols and identity
####################

Two symbols are used to represent discrete operations:

.. math::

   \dif{q}
   &
   \equiv
   q^{n+1}
   -
   q^{n  },

   \ave{q}
   &
   \equiv
   \frac{1}{2}
   q^{n+1}
   +
   \frac{1}{2}
   q^{n  },

which are the central differentiation and the central interpolation at the intermediate time step :math:`n + \frac{1}{2}`, respectively.

The discrete derivative of the trigonometric functions are

.. math::

   \dder{}{t}
   \cos \pos
   &
   =
   \frac{
      \cos \pos^{n+1}
      -
      \cos \pos^{n  }
   }{
      \dif{t}
   }

   &
   =
   \frac{
      \cos \left( \ave{\pos} + \frac{\dif{\pos}}{2} \right)
      -
      \cos \left( \ave{\pos} - \frac{\dif{\pos}}{2} \right)
   }{
      \dif{t}
   }

   &
   =
   -
   \frac{2}{\dif{t}}
   \sin \frac{\dif{\pos}}{2}
   \sin \ave{\pos}

   &
   =
   -
   \frac{2}{\dif{t}}
   \frac{\dif{\pos}}{2}
   \text{sinc} \frac{\dif{\pos}}{2}
   \sin \ave{\pos}

   &
   =
   -
   \ave{\vel}
   \text{sinc} \frac{\dif{\pos}}{2}
   \sin \ave{\pos},

.. math::

   \dder{}{t}
   \sin \pos
   &
   =
   \frac{
      \sin \pos^{n+1}
      -
      \sin \pos^{n  }
   }{
      \dif{t}
   }

   &
   =
   \frac{
      \sin \left( \ave{\pos} + \frac{\dif{\pos}}{2} \right)
      -
      \sin \left( \ave{\pos} - \frac{\dif{\pos}}{2} \right)
   }{
      \dif{t}
   }

   &
   =
   \frac{2}{\dif{t}}
   \sin \frac{\dif{\pos}}{2}
   \cos \ave{\pos}

   &
   =
   \frac{2}{\dif{t}}
   \frac{\dif{\pos}}{2}
   \text{sinc} \frac{\dif{\pos}}{2}
   \cos \ave{\pos}

   &
   =
   \ave{\vel}
   \text{sinc} \frac{\dif{\pos}}{2}
   \cos \ave{\pos}.

