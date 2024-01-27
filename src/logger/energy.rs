#![deny(missing_docs)]

//! Calculate the instantaneous energies of the given pendulum.

/// Calculate energies of the current state.
///
/// The kinetic and the potential energies are computed following [the definition](https://naokihori.github.io/Pendulum/discrete/energy_conservation.html).
/// The (non-dimensional) potential energy is computed with repsect to the quiescent state.
/// Since I fix the initial condition in [`crate::simulator::init`], the expected total energy is known a propri.
/// This value is used to normalise the total value.
pub fn check(p: &crate::Pendulum) -> crate::Energy {
    let nitems: usize = p.nitems;
    let poss: &Vec<f64> = &p.poss;
    let vels: &Vec<f64> = &p.vels;
    let mut t = 0.;
    let mut u = 0.5 * nitems as f64 * (nitems + 1) as f64;
    for i in 0..nitems {
        for j in 0..nitems {
            // kinetic energy
            t += 0.5
                * (nitems - if i > j { i } else { j }) as f64
                * vels[i]
                * vels[j]
                * (poss[i] - poss[j]).cos();
        }
        // potential energy
        u -= (nitems - i) as f64 * poss[i].sin()
    }
    // normalised by the analytical total energy
    t /= (nitems * (nitems + 1)) as f64;
    u /= (nitems * (nitems + 1)) as f64;
    return crate::Energy { t, u };
}
