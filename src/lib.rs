//! Simulates N-body pendulum.

pub mod pendulum;

#[cfg(not(feature = "binary"))]
use wasm_bindgen::prelude::*;

pub use crate::pendulum::{Energy, Pendulum};

/// Is invoked when the wasm file is loaded by JS.
#[cfg_attr(not(feature = "binary"), wasm_bindgen(start))]
pub fn init() {}
