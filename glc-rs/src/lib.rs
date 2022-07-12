mod utils;

use cubic_spline::{Point, Points, SplineOpts, TryFrom};
use lazy_static::lazy_static;
use mut_static::MutStatic;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

// Snippet from https://rustwasm.github.io/wasm-bindgen/examples/console-log.html
#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

macro_rules! console_log {
	($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn greet() {
    // alert("Hello from glc-rs!");
    console_log!("Hello from glc-rs!");
}

#[wasm_bindgen]
pub fn print_number(a: f32) {
    console_log!("Number {}", a);
}

#[wasm_bindgen]
pub fn print_coords(x: f32, y: f32) {
    console_log!("x: {}, y: {}", x, y);
}

struct Coordinates {
    points: Vec<Point>,
}

impl Coordinates {
    pub fn new() -> Self {
        Coordinates {
            points: (Vec::new()),
        }
    }

    pub fn add_coord(&mut self, x: f32, y: f32) {
        self.points.push(Point {
            x: x as f64,
            y: y as f64,
            tension: None,
        });
    }

    pub fn set_coord(&mut self, idx: usize, y: f32) {
        self.points[idx].y = y as f64;
    }
}

lazy_static! {
    static ref COORDS: MutStatic<Coordinates> = MutStatic::new();
}

pub fn new_coords() {
    if !COORDS.is_set().expect("Error with MutStatic variable") {
        COORDS
            .set(Coordinates::new())
            .expect("Could not create new Coordinates");
    }
}

#[wasm_bindgen]
pub fn add_coords(x: f32, y: f32) {
    new_coords();
    let mut coords = COORDS.write().unwrap();
    coords.add_coord(x, y);
}

#[wasm_bindgen]
pub fn set_coords(idx: usize, y: f32) {
    new_coords();
    let mut coords = COORDS.write().unwrap();
    coords.set_coord(idx, y);
}

#[wasm_bindgen]
pub fn print_all_coords() {
    new_coords();
    let coords = COORDS.read().unwrap();
    for p in coords.points.iter() {
        print_coords(p.x as f32, p.y as f32);
    }
}

#[wasm_bindgen]
pub fn build_spline() {
    let options = SplineOpts::new().tension(0.2);

    let coords = COORDS.read().unwrap();

    let points = Points::try_from(&(coords.points)).expect("Could not convert coordinates");

    let spline = points
        .calc_spline(&options)
        .expect("Could not calculate spline");

    for point in spline.get_ref() {
        console_log!("x: {}, y: {}", point.x, point.y);
    }
}
