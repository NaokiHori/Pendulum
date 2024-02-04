#![deny(missing_docs)]

//! An energy-conserving integrator.

/// Solves a linear system: A x = b.
///
/// I assume `x` contains the previous solution of the system.
/// The result A^{-1} b is first stored in `b`, which are compared with `x` to measure how close to the convergence the system is.
#[cfg(not(feature = "explicit"))]
fn solve_linear_system(nitems: usize, a: &mut Vec<f64>, x: &mut Vec<f64>, b: &mut Vec<f64>) -> f64 {
    // Classic Gaussian elimination.
    {
        // forward elimination
        for i in 0..nitems {
            for ii in i + 1..nitems {
                let v: f64 = a[ii * nitems + i] / a[i * nitems + i];
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
    // now `b` stores the solution
    // check convergence and update `x` by copying `b`
    let mut residual: f64 = 0.;
    for i in 0..nitems {
        residual += (b[i] - x[i]).abs();
        x[i] = b[i];
    }
    return residual;
}

/// Core function of the integrator.
///
/// This function solves [the discrete governing equations](https://naokihori.github.io/Pendulum/discrete/time_marcher.html).
/// The main purpose is to iteratively update the difference of the angular velocities `dv` until
/// converged.
#[cfg(not(feature = "explicit"))]
fn kernel(dt: f64, nitems: usize, poss: &mut Vec<f64>, vels: &mut Vec<f64>) -> bool {
    // sinc function: sin(x) / x
    let sinc: fn(f64) -> f64 = |x: f64| -> f64 {
        if 0. == x {
            1.
        } else {
            x.sin() / x
        }
    };
    /// Computes updated angular velocity and the position of a point.
    ///
    /// To update the angle, the Crank-Nocolson scheme is adopted.
    fn get_new_values(v_old: f64, a_old: f64, dv: f64, dt: f64) -> [f64; 2] {
        let v_new: f64 = v_old + dv;
        let a_new: f64 = a_old + 0.5 * (v_old + v_new) * dt;
        return [v_new, a_new];
    }
    // initialise local arrays
    // difference of vels
    let mut dvels: Vec<f64> = vec![0.; nitems];
    // right-hand-side vector
    let mut rhs: Vec<f64> = vec![0.; nitems];
    // left-hand-side array
    let mut lhs: Vec<f64> = vec![0.; nitems * nitems];
    // Crank-Nicolson iteration
    // set max number of interations
    // TODO: this is purely ad-hoc
    let itermax: usize = nitems << 1;
    for _iter in 0..itermax {
        // compute LHS array and RHS vector
        for i in 0..nitems {
            // expand i-th point information
            let vi_old: f64 = vels[i];
            let ai_old: f64 = poss[i];
            let dveli: f64 = dvels[i];
            let [vi_new, ai_new]: [f64; 2] = get_new_values(vi_old, ai_old, dveli, dt);
            // potential energy contribution
            rhs[i] = (nitems - i) as f64
                * sinc(0.5 * ai_new - 0.5 * ai_old)
                * (0.5 * ai_new + 0.5 * ai_old).cos()
                * dt;
            // interactive effects
            for j in 0..nitems {
                // expand j-th point information
                let vj_old: f64 = vels[j];
                let aj_old: f64 = poss[j];
                let dvelj: f64 = dvels[j];
                let [vj_new, aj_new]: [f64; 2] = get_new_values(vj_old, aj_old, dvelj, dt);
                // mass matrix
                let m: f64 = nitems as f64 - (if i > j { i } else { j }) as f64;
                let cij_old: f64 = if i == j { 1. } else { (ai_old - aj_old).cos() };
                let cij_new: f64 = if i == j { 1. } else { (ai_new - aj_new).cos() };
                let numer: f64 = f64::EPSILON + (0.5 * vi_new * cij_new + 0.5 * vi_old * cij_old);
                let denom: f64 =
                    f64::EPSILON + (0.5 * vi_new + 0.5 * vi_old) * (0.5 * cij_new + 0.5 * cij_old);
                let cor: f64 = 0.5 + 0.5 * numer / denom;
                lhs[i * nitems + j] = m * cor * (0.5 * cij_new + 0.5 * cij_old);
                // kinetic energy contribution
                let vj: f64 = 0.5 * vj_new + 0.5 * vj_old;
                rhs[i] -= m
                    * vj
                    * vj
                    * sinc(0.5 * ai_new - 0.5 * ai_old - 0.5 * aj_new + 0.5 * aj_old)
                    * (0.5 * ai_new + 0.5 * ai_old - 0.5 * aj_new - 0.5 * aj_old).sin()
                    * dt;
            }
        }
        // solve Ax = b, where
        //   A: lhs
        //   x: dvels
        //   b: rhs
        let residual: f64 = solve_linear_system(nitems, &mut lhs, &mut dvels, &mut rhs);
        // terminate iteration when converged
        if f64::EPSILON > residual {
            // finally update information
            for i in 0..nitems {
                const PI: f64 = std::f64::consts::PI;
                let [v, mut p]: [f64; 2] = get_new_values(vels[i], poss[i], dvels[i], dt);
                if p < -PI {
                    p += 2. * PI;
                }
                if PI < p {
                    p -= 2. * PI;
                }
                vels[i] = v;
                poss[i] = p;
            }
            return true;
        }
    }
    // not converged
    return false;
}

/// Entry point of the integrator.
///
/// There is no guarantee that the integrator converges with the given time step size `dt`.
/// As a remedy, I try to integrate the equation anyway, and make `dt` halved if it turns out to fail.
/// With this approach, however, `dt` gets smaller and smaller as proceeds.
/// To avoid unnecessarily small `dt`, I first double it.
#[cfg(not(feature = "explicit"))]
pub fn entrypoint(p: &mut crate::Pendulum) -> f64 {
    let dt: &mut f64 = &mut p.dt;
    let nitems: usize = p.nitems;
    let mut poss: &mut Vec<f64> = &mut p.poss;
    let mut vels: &mut Vec<f64> = &mut p.vels;
    *dt *= 2.;
    loop {
        if kernel(*dt, nitems, &mut poss, &mut vels) {
            return *dt;
        }
        *dt *= 0.5;
    }
}
