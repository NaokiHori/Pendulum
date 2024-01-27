#![deny(missing_docs)]

//! Simulates N-body pendulums.
//!
//! This crate is designed to solve [the governing equations](https://naokihori.github.io/Pendulum/equation/main.html) describing the motion of N-body pendulums.
//! The central function is [`simulator::integrate()`], which integrates the system *for
//! one step*.
//! For instance,
//! ```rust
//! let mut p: pendulum::Pendulum = pendulum::simulator::init(nitems);
//! // call as many times as you want
//! let dt: f64 = pendulum::simulator::integrate(&mut p);
//! ```
//! Note that the time step size `dt` is *decided by the integrator* and not to be specified.

/// Stores the state of the pendulum.
pub struct Pendulum {
    /// Number of mass points.
    nitems: usize,
    /// Angles.
    poss: Vec<f64>,
    /// Angular velocities.
    vels: Vec<f64>,
    /// Time step size, which is used as a reference to decide the next value.
    dt: f64,
}

/// Getters.
impl Pendulum {
    /// Getter, number of items.
    pub fn get_nitems(&self) -> usize {
        return self.nitems;
    }
    /// Getter, positions (angles).
    pub fn get_positions(&self) -> &Vec<f64> {
        return &self.poss;
    }
    /// Getter, (angular) velocities.
    pub fn get_velocities(&self) -> &Vec<f64> {
        return &self.vels;
    }
}

/// Stores kinetic and the potential energies.
pub struct Energy {
    /// Kinetic energy
    pub t: f64,
    /// Potential energy
    pub u: f64,
}

/// Has methods to manipulate the pendulum.
pub mod simulator {
    mod init;
    mod integrate;
    /// Initialises a pendulum and return it.
    pub fn init(nitems: usize) -> crate::Pendulum {
        let p: crate::Pendulum = init::entrypoint(nitems);
        return p;
    }
    /// Integrates the pendulum in time for one time step.
    pub fn integrate(p: &mut crate::Pendulum) -> f64 {
        // time step size is determined by the solver
        let dt: f64 = integrate::entrypoint(p);
        return dt;
    }
}

/// Calculates the current statistics of the system.
pub mod logger {
    mod energy;
    /// Calculates current energies.
    pub fn check_energies(p: &crate::Pendulum) -> crate::Energy {
        return energy::check(p);
    }
}
