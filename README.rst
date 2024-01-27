###################################################
`Pendulum <https://naokihori.github.io/Pendulum/>`_
###################################################

|License|_ |LastCommit|_ |CI|_

.. |License| image:: https://img.shields.io/github/license/NaokiHori/Pendulum
.. _License: https://opensource.org/licenses/MIT

.. |LastCommit| image:: https://img.shields.io/github/last-commit/NaokiHori/Pendulum/main
.. _LastCommit: https://github.com/NaokiHori/Pendulum/commits/main

.. |CI| image:: https://github.com/NaokiHori/Pendulum/actions/workflows/ci.yml/badge.svg?branch=main
.. _CI: https://github.com/NaokiHori/Pendulum/actions/workflows/ci.yml

.. image:: https://img.shields.io/badge/youtube-%23EE4831.svg?&style=for-the-badge&logo=youtube&logoColor=white
   :target: https://youtu.be/UQg8x4yleWE
   :width: 10%

.. image:: https://github.com/NaokiHori/Pendulum/blob/main/web/sphinx/thumbnail.gif
   :target: https://youtu.be/UQg8x4yleWE
   :width: 100%

********
Overview
********

This library simulates the motion of N-body pendulums.

By using an interesting yet a very simple topic, the main objective is to create a project where

* the main simulator,

* the background information (the equations, the numerical methods), and

* the user interface (the visualiser)

are all included and properly combined.

***********
Quick start
***********

======================
Try it on your browser
======================

Visit `the website <https://naokihori.github.io/Pendulum/>`_, where an on-line simulator is available (note that it consumes some amount of your CPU resource).

The simulator and the surrounding interfaces are implemented in ``Rust``, which are converted into ``WebAssembly`` for real-time executions powered by `wasm-bindgen <https://rustwasm.github.io/docs/wasm-bindgen/>`_.
Please check out `the other project <https://github.com/NaokiHori/MinimalCanvasExample>`_ for more details.

`All recent browsers should be supported in theory <https://webassembly.org/features/>`_.
The followings are checked at least:

* Chrome 120.0.6099.119

* Safari 17.1

===============
Try it locally
===============

Things put at the root level (``Cargo.toml`` and ``src``) serve to construct a simple simulator written in ``Rust``.
One can try it locally and easily, without touching web-related things (``WebAssembly``, ``Sphinx`` documentation, ``TypeScript``) at all.
After cloning this project:

.. code-block:: console

   git clone https://github.com/NaokiHori/Pendulum

execute:

.. code-block:: console

   cargo run --release

A dumped file ``energy.dat`` shows how the kinetic (2nd column), the potential (3rd column), and the total (4th column) energies change in time (1st column).
It is a simple ``txt`` file basically; use whatever you like (e.g., ``Gnuplot``, ``Matplotlib``) to plot the results.

*****************
Physical interest
*****************

One of the physical-side topics which I am of interest in this project is `the conservation of the total energy <https://naokihori.github.io/Pendulum/equation/main.html>`_.
Since I assume that the pendulums are frictionless, the total energy should be conserved throughout the run.
Although it is trivial in theory, achieving this property is not easy at all; in particular one needs to design a special scheme to this end.

Visit `the documentation <https://naokihori.github.io/Pendulum/>`_ to see how `the numerical methods <https://naokihori.github.io/Pendulum/discrete/main.html>`_ should be and how different `the results <https://naokihori.github.io/Pendulum/example/main.html>`_ are.

`An article written in Japanese <https://qiita.com/NaokiHori/items/736cf183c20eb2e91247>`_ (mostly focusing on the physical side) is available as well.

****************
C implementation
****************

Initially this project adopted C, which was removed from this repository and is now placed `here <https://gitlab.com/NaokiHori/pendulum>`_ (the code is still about 10% faster).

