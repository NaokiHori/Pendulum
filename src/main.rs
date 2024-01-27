#![deny(missing_docs)]

//! Simulates N-body pendulums.

/// Serves as the entry point of the binary crate.
fn main() -> () {
    // maximum time of the run
    let timemax: f64 = 1e+3;
    // dump energy per this time units
    let stat_rate: f64 = 1e-1;
    // number of bodies: primary parameter of this simulator
    let nitems: usize = 8;
    // obtain the initial state of a pendulum
    let mut p: pendulum::Pendulum = pendulum::simulator::init(nitems);
    // start to integrate the equations in time
    let mut time: f64 = 0.;
    let mut stat_next: f64 = stat_rate;
    loop {
        // the simulator returns time-step size it proceeds
        let dt: f64 = pendulum::simulator::integrate(&mut p);
        time += dt;
        if timemax < time {
            break;
        }
        if stat_next < time {
            check_stats(time, &p);
            stat_next += stat_rate;
            println!("time: {:5.2e}, dt: {:5.2e}", time, dt);
        }
    }
}

/// Writes the instantaneous energies to a file.
fn check_stats(time: f64, p: &pendulum::Pendulum) -> () {
    const FILENAME: &'static str = "energy.dat";
    let e: pendulum::Energy = pendulum::logger::check_energies(p);
    let string: String = format!(
        "{:+18.15e} {:+18.15e} {:+18.15e} {:+18.15e}",
        time,
        e.t,
        e.u,
        e.t + e.u
    );
    let mut file: std::fs::File = match std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(FILENAME)
    {
        Ok(file) => file,
        Err(e) => {
            eprintln!("\"{}\": {}", FILENAME, e);
            return;
        }
    };
    use std::io::Write;
    match writeln!(file, "{}", string) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("\"{}\": {}", FILENAME, e);
            return;
        }
    };
}
