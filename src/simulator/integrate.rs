#![deny(missing_docs)]

//! Integrates the equation to update a pendulum for one step.

mod implicit;

/// Invokes the normal integrator.
#[cfg(not(feature = "explicit"))]
pub fn entrypoint(p: &mut crate::Pendulum) -> f64 {
    return implicit::entrypoint(p);
}

mod explicit;

/// Invokes the explicit integrator, which is not energy-conserving.
#[cfg(feature = "explicit")]
pub fn entrypoint(p: &mut crate::Pendulum) -> f64 {
    return explicit::entrypoint(p);
}
