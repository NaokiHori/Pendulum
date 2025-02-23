# [Pendulum](https://naokihori.github.io/Pendulum/)

![License](https://img.shields.io/github/license/NaokiHori/Pendulum)
[![Last Commit](https://img.shields.io/github/last-commit/NaokiHori/Pendulum/main)](https://github.com/NaokiHori/Pendulum/commits/main)
[![CI](https://github.com/NaokiHori/Pendulum/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/NaokiHori/Pendulum/actions/workflows/ci.yml)
[![Pages](https://github.com/NaokiHori/Pendulum/actions/workflows/pages.yml/badge.svg?branch=main)](https://github.com/NaokiHori/Pendulum/actions/workflows/pages.yml)

[![YouTube](https://img.shields.io/badge/youtube-%23EE4831.svg?&style=for-the-badge&logo=youtube&logoColor=white)](https://youtu.be/UQg8x4yleWE)

[![Thumbnail](https://github.com/NaokiHori/Pendulum/blob/main/docs/source/thumbnail.gif)](https://youtu.be/UQg8x4yleWE)

## Quick Start

Visit [the website](https://naokihori.github.io/Pendulum/), where an online simulator is available.

The simulator and the surrounding interfaces are implemented in `Rust`, which are converted into `WebAssembly` for real-time executions powered by [`wasm-bindgen`](https://rustwasm.github.io/docs/wasm-bindgen/).

[All recent browsers should be supported in theory](https://webassembly.org/features/).
At least I have checked:

- Chrome 120.0.6099.119
- Safari 17.1
- Microsoft Edge 121.0.2277.83
- Firefox 111.0.1

## Physical Interest

One of the physical topics of interest in this project is [the conservation of total energy](https://naokihori.github.io/Pendulum/docs/equation/main.html).
Since the pendulums are assumed frictionless, the total energy should be conserved throughout the run.
Although trivial in theory, achieving this property is challenging and requires a special scheme.

Visit [the documentation](https://naokihori.github.io/Pendulum/docs) to see how [the numerical methods](https://naokihori.github.io/Pendulum/docs/discrete/main.html) work and how different [the results](https://naokihori.github.io/Pendulum/docs/example/main.html) are.

## 日本語記事（An article written in Japanese）

[エネルギ保存性に関して述べた記事](https://qiita.com/NaokiHori/items/736cf183c20eb2e91247)をQiitaに投稿しました。
