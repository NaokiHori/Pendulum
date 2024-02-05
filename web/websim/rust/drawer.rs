use wasm_bindgen::prelude::*;

const WHITE: &'static str = "#ffffff";
const RED: &'static str = "#ff0000";
const BLUE: &'static str = "#00ffff";
const YELLOW: &'static str = "#ffff88";

/// Stores pointers to HTML elements.
pub struct Drawer {
    canvas: web_sys::HtmlCanvasElement,
    context: web_sys::CanvasRenderingContext2d,
}

impl Drawer {
    /// Gets HTML elements are hold them.
    pub fn new() -> Drawer {
        let document: web_sys::Document = web_sys::window().unwrap().document().unwrap();
        let canvas: web_sys::HtmlCanvasElement = document
            .get_element_by_id("my-canvas")
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();
        let context: web_sys::CanvasRenderingContext2d = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();
        return Drawer { canvas, context };
    }
    /// Core function to draw all to canvas.
    ///
    /// * `nitems` - number of mass points of the pendulum.
    /// * `angles` - rotational angles of the mass points.
    /// * `tpoints`- trajectory of the last mass point.
    /// * `enes`   - history of the energies.
    pub fn draw(
        &self,
        nitems: usize,
        angles: &Vec<f64>,
        tpoints: &Vec<[f64; 2]>,
        enes: &[Vec<f64>; 2],
    ) -> () {
        let canvas: &web_sys::HtmlCanvasElement = &self.canvas;
        let context: &web_sys::CanvasRenderingContext2d = &self.context;
        // get canvas size
        let w: f64 = canvas.client_width() as f64;
        let h: f64 = canvas.client_height() as f64;
        // clean canvas
        context.clear_rect(0., 0., w, h);
        let show_graph: bool = 2. * h < w;
        // 1. pendulum, left-most of canvas
        {
            // prepare linear mappings from physical to screen domains
            // x: physical, y: screen
            // horizontal scaling
            let rescale_h = |x: f64| -> f64 {
                let xmin: f64 = -1. * nitems as f64 - 0.25;
                let xmax: f64 = nitems as f64 + 0.25;
                let ymin: f64 = if show_graph { 0. } else { 0.5 * w - 0.5 * h };
                let ymax: f64 = if show_graph { h } else { 0.5 * w + 0.5 * h };
                return ymin + (x - xmin) / (xmax - xmin) * (ymax - ymin);
            };
            // vertical scaling
            let rescale_v = |x: f64| -> f64 {
                let xmin: f64 = -1. * nitems as f64 - 0.25;
                let xmax: f64 = nitems as f64 + 0.25;
                let ymin: f64 = 0.;
                let ymax: f64 = h;
                return ymin + (x - xmin) / (xmax - xmin) * (ymax - ymin);
            };
            // NOTE: order matters
            let linewidth: f64 = h / 100.;
            draw_trajectory(context, rescale_h, rescale_v, linewidth, tpoints);
            let linewidth: f64 = h / 200.;
            draw_rods(context, rescale_h, rescale_v, linewidth, angles);
            let linewidth: f64 = h / 15. / nitems as f64;
            draw_mass_points(context, rescale_h, rescale_v, linewidth, angles);
        }
        // 2. graph, right-most part
        if show_graph {
            draw_graph(context, w, h, enes);
        }
    }

    /// Updates canvas size when the window is resized.
    pub fn update_canvas_size(&self) -> () {
        let canvas: &web_sys::HtmlCanvasElement = &self.canvas;
        let w: i32 = canvas.client_width();
        let h: i32 = canvas.client_height();
        canvas.set_width(w as u32);
        canvas.set_height(h as u32);
    }
}

/// Draws a trajectory of the last mass point.
fn draw_trajectory<F0, F1>(
    context: &web_sys::CanvasRenderingContext2d,
    rescale_h: F0,
    rescale_v: F1,
    linewidth: f64,
    tpoints: &Vec<[f64; 2]>,
) -> ()
where
    F0: Fn(f64) -> f64,
    F1: Fn(f64) -> f64,
{
    let nitems: usize = tpoints.len();
    for n in 0..nitems - 1 {
        let rate: f64 = (n as f64 / nitems as f64).powi(2i32);
        // gradually change linewidth
        context.set_line_width(rate * linewidth);
        // gradually change color, from black (old) to white (new)
        let color: u8 = (255. * rate) as u8;
        let color: String = format!("#{color:02x}{color:02x}{color:02x}");
        context.set_stroke_style(&JsValue::from_str(&color));
        // pick-up start and end
        let s: [f64; 2] = tpoints[n + 0];
        let e: [f64; 2] = tpoints[n + 1];
        context.begin_path();
        context.move_to(rescale_h(s[0]), rescale_v(s[1]));
        context.line_to(rescale_h(e[0]), rescale_v(e[1]));
        context.stroke();
    }
}

/// Draws lines through all mass points.
fn draw_rods<F0, F1>(
    context: &web_sys::CanvasRenderingContext2d,
    rescale_h: F0,
    rescale_v: F1,
    linewidth: f64,
    angles: &Vec<f64>,
) -> ()
where
    F0: Fn(f64) -> f64,
    F1: Fn(f64) -> f64,
{
    context.set_stroke_style(&JsValue::from_str(WHITE));
    context.set_line_width(linewidth);
    let mut x: f64 = 0.;
    let mut y: f64 = 0.;
    context.begin_path();
    context.move_to(rescale_h(x), rescale_v(y));
    for angle in angles {
        x += angle.cos();
        y += angle.sin();
        context.line_to(rescale_h(x), rescale_v(y));
    }
    context.stroke();
}

/// Draws filled circles at all mass points.
fn draw_mass_points<F0, F1>(
    context: &web_sys::CanvasRenderingContext2d,
    rescale_h: F0,
    rescale_v: F1,
    linewidth: f64,
    angles: &Vec<f64>,
) -> ()
where
    F0: Fn(f64) -> f64,
    F1: Fn(f64) -> f64,
{
    context.set_fill_style(&JsValue::from_str(YELLOW));
    let mut x: f64 = 0.;
    let mut y: f64 = 0.;
    const ARCS: [f64; 2] = [0., 2. * std::f64::consts::PI];
    for angle in angles {
        x += angle.cos();
        y += angle.sin();
        context.begin_path();
        context
            .arc(rescale_h(x), rescale_v(y), linewidth, ARCS[0], ARCS[1])
            .unwrap();
        context.fill();
    }
}

/// Draws a graph at the right-most position in the canvas.
fn draw_graph(
    context: &web_sys::CanvasRenderingContext2d,
    w: f64,
    h: f64,
    enes: &[Vec<f64>; 2],
) -> () {
    // use right-most part of the canvas
    let rescale_h = |x: f64| -> f64 {
        // assume data is normalised
        let xmin: f64 = 0.;
        let xmax: f64 = 1.;
        // make left-right margins
        let ymin: f64 = h + 0.15 * h;
        let ymax: f64 = w - 0.05 * h;
        return ymin + (x - xmin) / (xmax - xmin) * (ymax - ymin);
    };
    let rescale_v = |x: f64| -> f64 {
        // assume data is normalised
        let xmin: f64 = 0.;
        let xmax: f64 = 1.;
        // make top-bottom margins
        // NOTE: canvas y coordinate is flipped
        let ymin: f64 = 0.95 * h;
        let ymax: f64 = 0.05 * h;
        return ymin + (x - xmin) / (xmax - xmin) * (ymax - ymin);
    };
    // axes
    {
        context.set_stroke_style(&JsValue::from_str(WHITE));
        context.set_line_width(h / 200.);
        // x
        context.begin_path();
        context.move_to(rescale_h(0.), rescale_v(0.));
        context.line_to(rescale_h(1.), rescale_v(0.));
        context.stroke();
        // y
        context.begin_path();
        context.move_to(rescale_h(0.), rescale_v(0.));
        context.line_to(rescale_h(0.), rescale_v(1.));
        context.stroke();
    }
    // axes labels
    {
        context.set_font("5vw Arial");
        context.set_text_baseline("middle");
        context.set_text_align("left");
        // "T"
        context.set_fill_style(&JsValue::from_str(RED));
        context.fill_text("T", h, 0.25 * h).unwrap();
        // "U"
        context.set_fill_style(&JsValue::from_str(BLUE));
        context.fill_text("U", h, 0.75 * h).unwrap();
    }
    // energies as a function of time
    {
        let kernel = |data: &Vec<f64>, style: &str| -> () {
            // scale data to canvas and plot
            let nitems: usize = data.len();
            context.set_stroke_style(&JsValue::from_str(style));
            context.set_line_width(h / 50.);
            context.begin_path();
            let x: f64 = rescale_h(0.);
            let y: f64 = rescale_v(data[0]);
            context.move_to(x, y);
            for n in 1..nitems {
                let x: f64 = rescale_h(n as f64 / nitems as f64);
                let y: f64 = rescale_v(data[n]);
                context.line_to(x, y);
            }
            context.stroke();
        };
        kernel(&enes[0], RED);
        kernel(&enes[1], BLUE);
    }
}
