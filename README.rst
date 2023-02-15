########
Pendulum
########

|License|_ |LastCommit|_ |CI|_ |Documentation|_

.. |License| image:: https://img.shields.io/github/license/NaokiHori/Pendulum
.. _License: https://opensource.org/licenses/MIT

.. |LastCommit| image:: https://img.shields.io/github/last-commit/NaokiHori/Pendulum/main
.. _LastCommit: https://github.com/NaokiHori/Pendulum/commits/main

.. |CI| image:: https://github.com/NaokiHori/Pendulum/actions/workflows/ci.yml/badge.svg?branch=main
.. _CI: https://github.com/NaokiHori/Pendulum/actions/workflows/ci.yml

.. |Documentation| image:: https://github.com/NaokiHori/Pendulum/actions/workflows/documentation.yml/badge.svg?branch=main
.. _Documentation: https://github.com/NaokiHori/Pendulum/actions/workflows/documentation.yml

.. image:: https://img.shields.io/badge/youtube-%23EE4831.svg?&style=for-the-badge&logo=youtube&logoColor=white
   :target: https://youtu.be/UQg8x4yleWE
   :width: 10%

.. image:: https://github.com/NaokiHori/Pendulum/blob/main/docs/source/thumbnail.png
   :target: https://youtu.be/UQg8x4yleWE
   :width: 100%

.. image:: https://github.com/NaokiHori/Pendulum/blob/main/docs/source/thumbnail.gif
   :width: 100%

********
Overview
********

This library simulates the motion of a pendulum with N masses and simultaneously visualises the motion using `the X Window System <https://x.org/wiki/>`_.

Please refer to `the documentation <https://naokihori.github.io/Pendulum/>`_ for the governing equations and the numerical treatments.

**********
Motivation
**********

   * Just for fun.

   * Trying to use the X Window System.

   * Trying pseudo-OOP in C.

**********
Dependency
**********

========
Required
========

   * C compiler

   * GNU make

===========
Recommended
===========

   * Python (with ``NumPy`` and ``Matplotlib`` for the visualisation of the results)

   * X Window System (optional)

***********
Quick start
***********

Since the motion of the pendulum is given by an ordinary differential equation, we need an initial condition, which is defined in ``src/main.c``.

=======================
Without ``X`` (default)
=======================

Without the simultaneous visualisation feature, this is a simple C program, so it is easy to compile and use:

.. code-block::

   $ make output
   $ make all
   $ ./a.out

The results (pendulum positions) are simply written to files under ``output/`` in ASCII, which can be visualised later by e.g.,

.. code-block::

   $ python3 visualise.py

=====================
With ``X`` (optional)
=====================

To enable the in-situ visualisation functionality, ``X`` must be properly installed and linked.
In particular, ``X11/Xlib.h`` and ``libX11.dylib`` must be included and linked (``-lX11``), respectively:

.. code-block::

   $ make -f Makefile.xwindow output
   $ make -f Makefile.xwindow all
   $ ./a.out

This opens the other window, which displays the instantaneous pendulum motion.

