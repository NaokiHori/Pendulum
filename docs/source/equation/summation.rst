First, the base case with :math:`N = 1` is true since

.. math::

   \sum_{\ia = 0}^{0}
   \sum_{\ib = 0}^{\ia}
   \sum_{\ic = 0}^{\ia}
   Q_{\ib \ic}
   =
   \sum_{\ib = 0}^{0}
   \sum_{\ic = 0}^{0}
   \left\{
      1 - \max \left( \ib, \ic \right)
   \right\}
   Q_{\ib \ic}
   =
   Q_{0 0}.

Now we assume that the statement is true for :math:`N` and consider with respect to :math:`N + 1`.

The left-hand-side term of the statement leads to

.. math::

   \sum_{\ia = 0}^{N}
   \sum_{\ib = 0}^{\ia}
   \sum_{\ic = 0}^{\ia}
   Q_{\ib \ic}
   =
   \sum_{\ia = 0}^{N - 1}
   \sum_{\ib = 0}^{\ia}
   \sum_{\ic = 0}^{\ia}
   Q_{\ib \ic}
   +
   \sum_{\ib = 0}^{N}
   \sum_{\ic = 0}^{N}
   Q_{\ib \ic},

while the right-hand-side term of the statement is

.. math::

   \sum_{\ib = 0}^{N}
   \sum_{\ic = 0}^{N}
   \left\{
      N + 1 - \max \left( \ib, \ic \right)
   \right\}
   Q_{\ib \ic}
   &
   =
   \sum_{\ib = 0}^{N - 1}
   \sum_{\ic = 0}^{N}
   \left\{
      N + 1 - \max \left( \ib, \ic \right)
   \right\}
   Q_{\ib \ic}
   +
   \sum_{\ic = 0}^{N}
   \left\{
      N + 1 - \max \left( N, \ic \right)
   \right\}
   Q_{N \ic}

   &
   =
   \sum_{\ib = 0}^{N - 1}
   \sum_{\ic = 0}^{N - 1}
   \left\{
      N + 1 - \max \left( \ib, \ic \right)
   \right\}
   Q_{\ib \ic}
   +
   \sum_{\ic = 0}^{N}
   \left\{
      N + 1 - \max \left( N, \ic \right)
   \right\}
   Q_{N \ic}
   +
   \sum_{\ib = 0}^{N - 1}
   \left\{
      N + 1 - \max \left( \ib, N \right)
   \right\}
   Q_{\ib N}

   &
   =
   \sum_{\ib = 0}^{N - 1}
   \sum_{\ic = 0}^{N - 1}
   \left\{
      N - \max \left( \ib, \ic \right)
   \right\}
   Q_{\ib \ic}
   +
   \sum_{\ib = 0}^{N - 1}
   \sum_{\ic = 0}^{N - 1}
   Q_{\ib \ic}
   +
   \sum_{\ic = 0}^{N}
   Q_{N \ic}
   +
   \sum_{\ib = 0}^{N - 1}
   Q_{\ib N}

   &
   =
   \sum_{\ib = 0}^{N - 1}
   \sum_{\ic = 0}^{N - 1}
   \left\{
      N - \max \left( \ib, \ic \right)
   \right\}
   Q_{\ib \ic}
   +
   \sum_{\ib = 0}^{N - 1}
   \sum_{\ic = 0}^{N}
   Q_{\ib \ic}
   +
   \sum_{\ic = 0}^{N}
   Q_{N \ic}

   &
   =
   \sum_{\ib = 0}^{N - 1}
   \sum_{\ic = 0}^{N - 1}
   \left\{
      N - \max \left( \ib, \ic \right)
   \right\}
   Q_{\ib \ic}
   +
   \sum_{\ib = 0}^{N}
   \sum_{\ic = 0}^{N}
   Q_{\ib \ic}.

Now the first terms are equal according to the assumption, while the second terms are identical.

