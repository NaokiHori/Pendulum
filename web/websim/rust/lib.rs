//! Bridges between JavaScript and [mod@wasm_bindgen].

mod drawer;

use wasm_bindgen::prelude::*;

/// Wrapper of objects containing states
#[wasm_bindgen]
pub struct WasmWrapper {
    /// Pendulum, nothing else.
    p: pendulum::Pendulum,
    /// History of the last mass point in Cartesian coordinate.
    trajs: Vec<[f64; 2]>,
    /// History of the kinetic and the potential energies.
    enes: [Vec<f64>; 2],
    /// Handles drawing, keeping HTML objects.
    drawer: crate::drawer::Drawer,
}

/// Maximum length of the trajectory ([`WasmWrapper::trajs`])
const NTRAJS: usize = 150;
/// Maximum length of the energy history ([`WasmWrapper::enes`])
const NENES: usize = 500;

/// Serves as an opaque pointer to store all states.
#[wasm_bindgen]
impl WasmWrapper {
    /// Initialiser.
    pub fn new(nitems: usize) -> WasmWrapper {
        // I need to initialise the size of the canvas first
        let drawer = crate::drawer::Drawer::new();
        drawer.update_canvas_size();
        // prepare all buffers, pack them and return it
        return WasmWrapper {
            p: pendulum::simulator::init(nitems),
            trajs: Vec::<[f64; 2]>::new(),
            enes: [Vec::<f64>::new(), Vec::<f64>::new()],
            drawer: drawer,
        };
    }
    /// Integrator.
    pub fn integrate(&mut self) -> () {
        // loop until desired time is reached
        const RATE: f64 = 5e-2;
        let mut time: f64 = 0.;
        loop {
            let dt: f64 = pendulum::simulator::integrate(&mut self.p);
            time += dt;
            if RATE < time {
                break;
            }
        }
        // save history
        fn update_history<T>(max: usize, old: &mut Vec<T>, new: T) -> () {
            // NOTE: VecDeque would be superior, saying strictly
            let nitems: usize = old.len();
            if max <= nitems {
                // long enough, discard the head and add the new to the tail
                old.rotate_left(1);
                old[max - 1] = new;
            } else {
                // short enough, just append the new to the tail
                old.push(new);
            }
        }
        // history of the last mass point
        // compute Cartesian coordinate of the last mass point
        let angles: &Vec<f64> = &self.p.get_positions();
        let mut x: f64 = 0.;
        let mut y: f64 = 0.;
        for angle in angles {
            x += angle.cos();
            y += angle.sin();
        }
        update_history::<[f64; 2]>(NTRAJS, &mut self.trajs, [x, y]);
        // history of the kinetic and the potential energies
        let e: pendulum::Energy = pendulum::logger::check_energies(&self.p);
        update_history::<f64>(NENES, &mut self.enes[0], e.t);
        update_history::<f64>(NENES, &mut self.enes[1], e.u);
    }
    /// Is a wrapper of the drawing function.
    pub fn draw(&self) -> () {
        self.drawer.draw(
            self.p.get_nitems(),
            &self.p.get_positions(),
            &self.trajs,
            &self.enes,
        );
    }
    /// Changes canvas size, invoked by the `resize` event.
    pub fn update_canvas_size(&self) -> () {
        self.drawer.update_canvas_size();
    }
}

/// Is invoked when the wasm file is loaded in JS.
#[wasm_bindgen(start)]
pub fn init() -> () {}
