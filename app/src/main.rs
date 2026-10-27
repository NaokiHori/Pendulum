#![deny(missing_docs)]

//! Simulates N-body pendulum.

use pendulum::{Energy, Pendulum};

/// Main function of the binary crate.
fn main() {
    // maximum time of the run
    let timemax: f64 = 1e+3;
    // dump energy per this time units
    let stat_rate: f64 = 1e-1;
    // number of masses
    let nitems: usize = 8;
    // obtain the initial state of a pendulum
    let mut pendulum = Pendulum::new(nitems);
    // start to integrate the equations in time
    let mut time: f64 = 0f64;
    let mut stat_next: f64 = stat_rate;
    loop {
        // time-step size is determined by the simulator
        let dt: f64 = pendulum.integrate();
        time += dt;
        if timemax < time {
            break;
        }
        if stat_next < time {
            check_stats(time, &pendulum);
            stat_next += stat_rate;
            println!("time: {:5.2e}, dt: {:5.2e}", time, dt);
        }
    }
}

/// Writes instantaneous energies to file.
fn check_stats(time: f64, pendulum: &Pendulum) {
    const FILE_NAME: &str = "energy.dat";
    let energy: Energy = pendulum.check_energies();
    let string: String = format!(
        "{:+18.15e} {:+18.15e} {:+18.15e} {:+18.15e}",
        time,
        energy.kinetic,
        energy.potential,
        energy.kinetic + energy.potential
    );
    let mut file: std::fs::File = match std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(FILE_NAME)
    {
        Ok(file) => file,
        Err(e) => {
            eprintln!("\"{}\": {}", FILE_NAME, e);
            return;
        }
    };
    use std::io::Write;
    match writeln!(file, "{}", string) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("\"{}\": {}", FILE_NAME, e);
        }
    };
}
