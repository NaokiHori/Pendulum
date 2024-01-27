#![deny(missing_docs)]

//! Initialises a pendulum.

/// Initialises a pendulum.
///
/// See [the example](https://naokihori.github.io/Pendulum/example/main.html) which explains why this specific condition is chosen.
pub fn entrypoint(nitems: usize) -> crate::Pendulum {
    let v0: f64 = (6. / (2. * nitems as f64 + 1.)).sqrt();
    let poss: Vec<f64> = vec![0.; nitems];
    let vels: Vec<f64> = vec![v0; nitems];
    let dt: f64 = 1.;
    return crate::Pendulum {
        nitems,
        poss,
        vels,
        dt,
    };
}
