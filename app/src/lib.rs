//! Simulates N-body pendulum.

pub mod pendulum;

#[cfg(feature = "library-crate")]
use wasm_bindgen::prelude::*;

pub use crate::pendulum::{Energy, Pendulum};

#[cfg_attr(feature = "library-crate", wasm_bindgen(start))]
pub fn init() {}
