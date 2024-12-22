//! Glue code.

pub mod pendulum;

pub use crate::pendulum::{Energy, Pendulum};
use wasm_bindgen::prelude::*;

/// Wraps objects which are needed by front-end.
#[wasm_bindgen]
pub struct Wrapper {
    /// Main object to be simulated.
    pendulum: Pendulum,
    /// Mass positions in Cartesian coordinates.
    positions: Vec<f64>,
}

/// Entry point.
#[wasm_bindgen]
impl Wrapper {
    /// Constructor.
    #[wasm_bindgen(constructor)]
    pub fn new(nitems: usize) -> Self {
        let pendulum = Pendulum::new(nitems);
        let positions = vec![0f64; nitems * 2];
        Self {
            pendulum,
            positions,
        }
    }

    /// Integrator.
    pub fn integrate(&mut self) -> f64 {
        let dt: f64 = self.pendulum.integrate();
        dt
    }

    /// Returns a pointer to the current mass positions in Cartesian coordinates.
    pub fn get_positions(&mut self) -> *const f64 {
        let angles: &[f64] = self.pendulum.get_positions();
        let positions: &mut [f64] = &mut self.positions;
        let mut x = 0f64;
        let mut y = 0f64;
        for (n, angle) in angles.iter().enumerate() {
            x += angle.cos();
            y += angle.sin();
            let x_index: usize = 2usize * n;
            let y_index: usize = 2usize * n + 1usize;
            positions[x_index] = x;
            positions[y_index] = y;
        }
        positions.as_ptr()
    }

    /// Calculate instantaneous kinetic and potential energies.
    pub fn check_energies(&self) -> Energy {
        self.pendulum.check_energies()
    }
}

/// Is invoked when the wasm file is loaded by JS.
#[wasm_bindgen(start)]
pub fn init() {}
