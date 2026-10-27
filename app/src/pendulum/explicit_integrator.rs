//! A naive integrator which is not energy-conserving.
//! This exists only to compare with the default implicit integrator, and thus is considered to be a feature.

const PI: f64 = std::f64::consts::PI;

/// Performs LDLT decomposition.
fn cholesky_decomposition(nitems: usize, a: &mut [f64], x: &mut [f64]) {
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

fn get_new_values(v_old: f64, a_old: f64, dv: f64, dt: f64) -> [f64; 2] {
    let v_new: f64 = v_old + dv;
    let a_new: f64 = a_old + v_new * dt;
    [v_new, a_new]
}

/// Core function of the integrator.
///
/// This function integrates the discrete governing equations in time by means of a fully-explicit scheme.
#[cfg(feature = "explicit")]
pub fn integrate(
    dt: f64,
    nitems: usize,
    velocities: &mut [f64],
    positions: &mut [f64],
    dvelocities: &mut [f64],
    lhs: &mut [f64],
) {
    // compute LHS array and RHS vector
    for i in 0..nitems {
        // RHS, potential energy contribution
        dvelocities[i] = (nitems - i) as f64 * positions[i].cos() * dt;
        // interactive effects
        for j in 0..nitems {
            let m: f64 = nitems as f64 - (if i > j { i } else { j }) as f64;
            let cij: f64 = if i == j {
                1.
            } else {
                (positions[i] - positions[j]).cos()
            };
            let sij: f64 = if i == j {
                0.
            } else {
                (positions[i] - positions[j]).sin()
            };
            // LHS array
            lhs[i * nitems + j] = m * cij;
            // RHS vector, kinetic energy contribution
            dvelocities[i] -= m * velocities[j] * velocities[j] * sij * dt;
        }
    }
    // solve linear system
    cholesky_decomposition(nitems, lhs, dvelocities);
    // update velocities and positions
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
}
