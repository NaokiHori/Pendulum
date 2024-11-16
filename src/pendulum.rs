#![deny(missing_docs)]

//! Simulates N-body pendulums.
//!
//! This crate is designed to solve [the governing equations](https://naokihori.github.io/Pendulum/docs/equation/main.html) describing the motion of N-body pendulums.
//! The core function is [`Pendulum::integrate()`], which integrates the system for one step.
//! For instance,
//! ```rust
//! let mut pendulum = Pendulum::new(nitems);
//! // call as many times as you want
//! let dt: f64 = pendulum.integrate();
//! ```
//! Note that the time step size `dt` is decided by the integrator and not to be specified.

//! Integrates the equation to update a pendulum for one step.
mod integrator {
    #[cfg(feature = "explicit")]
    pub mod explicit;
    #[cfg(not(feature = "explicit"))]
    pub mod implicit;
}

use wasm_bindgen::prelude::*;

/// Main struct, storing state of a pendulum and methods to update myself.
pub struct Pendulum {
    /// Number of masses.
    nitems: usize,
    /// Angular velocities.
    velocities: Vec<f64>,
    /// Angles.
    positions: Vec<f64>,
    /// Time step size, which is used as a reference to decide the next value.
    dt: f64,
    /// Internal vector buffer, 0.
    vec_buf0: Vec<f64>,
    /// Internal vector buffer, 1.
    #[cfg(not(feature = "explicit"))]
    vec_buf1: Vec<f64>,
    /// Internal vector buffers.
    arr_buf: Vec<f64>,
}

/// Stores kinetic and the potential energies.
#[wasm_bindgen]
pub struct Energy {
    /// Kinetic energy
    pub kinetic: f64,
    /// Potential energy
    pub potential: f64,
}

impl Pendulum {
    /// Constructor to initialize a pendulum.
    ///
    /// See [the example](https://naokihori.github.io/Pendulum/docs/example/main.html) which explains why this specific condition is chosen.
    pub fn new(nitems: usize) -> Self {
        let v0: f64 = (6f64 / (2 * nitems + 1) as f64).sqrt();
        let velocities = vec![v0; nitems];
        let positions = vec![0f64; nitems];
        let dt = 1f64;
        let vec_buf0 = vec![0f64; nitems];
        #[cfg(not(feature = "explicit"))]
        let vec_buf1 = vec![0f64; nitems];
        let arr_buf = vec![0f64; nitems * nitems];
        Self {
            nitems,
            positions,
            velocities,
            dt,
            vec_buf0,
            #[cfg(not(feature = "explicit"))]
            vec_buf1,
            arr_buf,
        }
    }

    /// Integrates the pendulum in time for one time step.
    pub fn integrate(&mut self) -> f64 {
        let nitems: usize = self.nitems;
        let dt: &mut f64 = &mut self.dt;
        let velocities: &mut Vec<f64> = &mut self.velocities;
        let positions: &mut Vec<f64> = &mut self.positions;
        let vec_buf0: &mut Vec<f64> = &mut self.vec_buf0;
        let arr_buf: &mut Vec<f64> = &mut self.arr_buf;
        #[cfg(not(feature = "explicit"))]
        {
            use integrator::implicit::integrate;
            let vec_buf1: &mut Vec<f64> = &mut self.vec_buf1;
            *dt *= 2f64;
            loop {
                if integrate(
                    *dt, nitems, velocities, positions, vec_buf0, vec_buf1, arr_buf,
                ) {
                    return *dt;
                }
                *dt *= 0.5f64;
            }
        }
        #[cfg(feature = "explicit")]
        {
            use integrator::explicit::integrate;
            // NOTE: ad-hoc way to decide `dt`
            *dt = 1e-3f64 / nitems as f64;
            integrate(*dt, nitems, velocities, positions, vec_buf0, arr_buf);
            *dt
        }
    }

    /// Calculate instantaneous kinetic and potential energies.
    ///
    /// The kinetic and the potential energies are computed following [the definition](https://naokihori.github.io/Pendulum/docs/equation/main.html).
    /// The (non-dimensional) potential energy is computed with repsect to the quiescent state.
    /// Since I fix the initial condition, the expected total energy is known a propri.
    /// This value is used to normalize the total value.
    pub fn check_energies(&self) -> Energy {
        let nitems: usize = self.nitems;
        let velocities: &Vec<f64> = &self.velocities;
        let positions: &Vec<f64> = &self.positions;
        let total: f64 = (nitems * (nitems + 1)) as f64;
        let kinetic = {
            let mut kinetic = 0f64;
            for i in 0..nitems {
                for j in 0..nitems {
                    kinetic += 0.5f64
                        * (nitems - if i < j { j } else { i }) as f64
                        * velocities[i]
                        * velocities[j]
                        * (positions[i] - positions[j]).cos();
                }
            }
            kinetic / total
        };
        let potential = {
            let mut potential = 0.5f64 * total;
            for (i, pos) in positions.iter().enumerate().take(nitems) {
                potential -= (nitems - i) as f64 * pos.sin()
            }
            potential / total
        };
        Energy { kinetic, potential }
    }

    /// Getter, number of masses.
    pub fn get_nitems(&self) -> usize {
        self.nitems
    }

    /// Getter, (angular) velocities.
    pub fn get_velocities(&self) -> &Vec<f64> {
        &self.velocities
    }

    /// Getter, positions (angles).
    pub fn get_positions(&self) -> &Vec<f64> {
        &self.positions
    }
}
