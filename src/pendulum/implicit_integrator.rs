#![deny(missing_docs)]

//! An energy-conserving integrator.

const PI: f64 = std::f64::consts::PI;

/// Computes (unnormalized) sinc function: sin(x) / x.
fn sinc(x: f64) -> f64 {
    if 0f64 - f64::EPSILON < x && x < 0f64 + f64::EPSILON {
        1f64
    } else {
        x.sin() / x
    }
}

/// Performs [Gaussian elimination](https://en.wikipedia.org/wiki/Gaussian_elimination).
fn gaussian_elimination(nitems: usize, a: &mut [f64], b: &mut [f64]) {
    // forward elimination
    for i in 0..nitems {
        let f: f64 = 1f64 / a[i * nitems + i];
        for ii in i + 1..nitems {
            let v: f64 = a[ii * nitems + i] * f;
            for j in i + 1..nitems {
                a[ii * nitems + j] -= v * a[i * nitems + j];
            }
            b[ii] -= v * b[i];
        }
    }
    // backward substitution
    for i in (0..nitems).rev() {
        for j in i + 1..nitems {
            b[i] -= a[i * nitems + j] * b[j];
        }
        b[i] /= a[i * nitems + i];
    }
}

/// Solves a linear system: A x = b.
///
/// `x` contains the previous solution of the system.
/// The result `A^{-1} b` is first stored in `b`, which is compared with `x` to evaluate the convevgence.
fn solve_linear_system(nitems: usize, a: &mut [f64], x: &mut [f64], b: &mut [f64]) -> f64 {
    // solve linear system by means of gaussian elimination
    gaussian_elimination(nitems, a, b);
    // now `b` stores the solution, which should be moved to `x`
    // check convergence and update `x` by copying `b`
    let mut residual: f64 = 0f64;
    for i in 0..nitems {
        residual += (b[i] - x[i]).abs();
        x[i] = b[i];
    }
    residual
}

/// Computes updated angular velocity and the position of a point.
///
/// To update the angle, the Crank-Nocolson scheme is adopted.
fn get_new_values(v_old: f64, p_old: f64, dv: f64, dt: f64) -> [f64; 2] {
    let v_new: f64 = v_old + dv;
    let p_new: f64 = p_old + 0.5f64 * (v_old + v_new) * dt;
    [v_new, p_new]
}

/// Core function of the integrator.
///
/// This function solves [the discrete governing equations](https://naokihori.github.io/Pendulum/discrete/time_marcher.html).  
/// The main purpose is to iteratively update the difference of the angular velocities `dv` until converged.
#[cfg(not(feature = "explicit"))]
pub fn integrate(
    dt: f64,
    nitems: usize,
    velocities: &mut [f64],
    positions: &mut [f64],
    dvelocities: &mut [f64],
    rhs: &mut [f64],
    lhs: &mut [f64],
) -> bool {
    dvelocities.fill(0f64);
    // Crank-Nicolson iteration
    // set max number of interations
    // TODO: purely ad-hoc
    let itermax: usize = nitems << 1;
    for _iter in 0..itermax {
        // compute LHS array and RHS vector
        for i in 0..nitems {
            // expand i-th point information
            let vi_old: f64 = velocities[i];
            let pi_old: f64 = positions[i];
            let dvi: f64 = dvelocities[i];
            let [vi_new, pi_new]: [f64; 2] = get_new_values(vi_old, pi_old, dvi, dt);
            let vi_mid: f64 = 0.5f64 * (vi_old + vi_new);
            let pi_mid: f64 = 0.5f64 * (pi_old + pi_new);
            let dpi_h: f64 = 0.5f64 * (-pi_old + pi_new);
            // potential energy contribution
            rhs[i] = (nitems - i) as f64 * sinc(dpi_h) * pi_mid.cos() * dt;
            // interactive effects
            for j in 0..nitems {
                // expand j-th point information
                let vj_old: f64 = velocities[j];
                let pj_old: f64 = positions[j];
                let dvj: f64 = dvelocities[j];
                let [vj_new, pj_new]: [f64; 2] = get_new_values(vj_old, pj_old, dvj, dt);
                let vj_mid: f64 = 0.5f64 * (vj_old + vj_new);
                let pj_mid: f64 = 0.5f64 * (pj_old + pj_new);
                let dpj_h: f64 = 0.5f64 * (-pj_old + pj_new);
                // mass matrix
                let mass: f64 = (nitems - if i < j { j } else { i }) as f64;
                let cij_old: f64 = if i == j {
                    1f64
                } else {
                    (pi_old - pj_old).cos()
                };
                let cij_new: f64 = if i == j {
                    1f64
                } else {
                    (pi_new - pj_new).cos()
                };
                let cor: f64 = {
                    let num: f64 = vi_old * cij_old + vi_new * cij_new;
                    let den: f64 = vi_mid * (cij_old + cij_new);
                    0.5f64 * (1f64 + num / den)
                };
                lhs[i * nitems + j] = mass * cor * 0.5f64 * (cij_old + cij_new);
                // kinetic energy contribution
                rhs[i] -=
                    mass * vj_mid.powi(2) * sinc(dpi_h - dpj_h) * (pi_mid - pj_mid).sin() * dt;
            }
        }
        // solve Ax = b, where
        //   A: lhs
        //   x: dvelocities
        //   b: rhs
        // NOTE: residual is L^1 normal and thus it is always positive
        let residual: f64 = solve_linear_system(nitems, lhs, dvelocities, rhs);
        // terminate iteration when converged
        if residual < f64::EPSILON {
            // finally update information
            for i in 0..nitems {
                let v: &mut f64 = &mut velocities[i];
                let p: &mut f64 = &mut positions[i];
                let dv: f64 = dvelocities[i];
                [*v, *p] = get_new_values(*v, *p, dv, dt);
                if *p < -PI {
                    *p += 2f64 * PI;
                }
                if PI < *p {
                    *p -= 2f64 * PI;
                }
            }
            return true;
        }
    }
    // not converged
    false
}
