//! A naive integrator which is not energy-conserving.
//! This exists only to compare with the default implicit integrator, and thus is considered to be
//! a feature.

#[cfg(feature = "explicit")]
fn solve_linear_system(nitems: usize, a: &mut Vec<f64>, x: &mut Vec<f64>, b: &mut Vec<f64>) -> () {
    fn cholesky_decomposition(nitems: usize, a: &mut Vec<f64>, x: &mut Vec<f64>) -> () {
        // NOTE: zero-division checks are omitted, assuming rank(A) is nitems
        // L D LT x = b
        //   D LT x = L^{-1}  b = y
        //     LT x = D^{-1}  y = z
        //        x = LT^{-1} z
        // LDLT decomposition
        for i in 0..nitems {
            // lower triangular matrix
            for j in 0..i {
                for k in 0..j {
                    let lik: f64 = a[i * nitems + k];
                    let ljk: f64 = a[j * nitems + k];
                    let dk: f64 = a[k * nitems + k];
                    a[i * nitems + j] -= lik * ljk * dk;
                }
                a[i * nitems + j] /= a[j * nitems + j];
            }
            // diagonal component
            for j in 0..i {
                let dj: f64 = a[j * nitems + j];
                let lij: f64 = a[i * nitems + j];
                a[i * nitems + i] -= lij * lij * dj;
            }
        }
        // eliminations
        // L y = b
        for i in 0..nitems {
            for j in 0..i {
                x[i] -= a[i * nitems + j] * x[j];
            }
        }
        // D z = y
        for i in 0..nitems {
            x[i] /= a[i * nitems + i];
        }
        // LT x = z
        for i in (0..nitems).rev() {
            for j in i + 1..nitems {
                x[i] -= a[j * nitems + i] * x[j];
            }
        }
    }
    cholesky_decomposition(nitems, a, b);
    // update x
    for i in 0..nitems {
        x[i] = b[i];
    }
}

#[cfg(feature = "explicit")]
fn kernel(dt: f64, nitems: usize, poss: &mut Vec<f64>, vels: &mut Vec<f64>) -> () {
    fn get_new_values(v_old: f64, a_old: f64, dv: f64, dt: f64) -> [f64; 2] {
        let v_new: f64 = v_old + dv;
        let a_new: f64 = a_old + v_new * dt;
        return [v_new, a_new];
    }
    // initialise local arrays
    // difference of vels
    let mut dvels: Vec<f64> = vec![0.; nitems];
    // right-hand-side vector
    let mut rhs: Vec<f64> = vec![0.; nitems];
    // left-hand-side array
    let mut lhs: Vec<f64> = vec![0.; nitems * nitems];
    // compute LHS array and RHS vector
    for i in 0..nitems {
        // RHS, potential energy contribution
        rhs[i] = (nitems - i) as f64 * poss[i].cos() * dt;
        // interactive effects
        for j in 0..nitems {
            let m: f64 = nitems as f64 - (if i > j { i } else { j }) as f64;
            let cij: f64 = if i == j {
                1.
            } else {
                (poss[i] - poss[j]).cos()
            };
            let sij: f64 = if i == j {
                0.
            } else {
                (poss[i] - poss[j]).sin()
            };
            // LHS array
            lhs[i * nitems + j] = m * cij;
            // RHS vector, kinetic energy contribution
            rhs[i] -= m * vels[j] * vels[j] * sij * dt;
        }
    }
    // solve linear system
    solve_linear_system(nitems, &mut lhs, &mut dvels, &mut rhs);
    // update vel and pos
    for i in 0..nitems {
        [vels[i], poss[i]] = get_new_values(vels[i], poss[i], dvels[i], dt);
    }
}

#[cfg(feature = "explicit")]
pub fn entrypoint(p: &mut crate::Pendulum) -> f64 {
    let dt: &mut f64 = &mut p.dt;
    let nitems: usize = p.nitems;
    let mut poss: &mut Vec<f64> = &mut p.poss;
    let mut vels: &mut Vec<f64> = &mut p.vels;
    // NOTE: ad-hoc way to decide `dt`
    *dt = 1e-3 / nitems as f64;
    kernel(*dt, nitems, &mut poss, &mut vels);
    return *dt;
}
