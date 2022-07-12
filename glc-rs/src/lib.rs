mod utils;

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
    xs: Vec<f32>,
    ys: Vec<f32>,
}

impl Coordinates {
    pub fn new() -> Self {
        Coordinates {
            xs: (Vec::new()),
            ys: (Vec::new()),
        }
    }

    pub fn add_coord(&mut self, x: f32, y: f32) {
        self.xs.push(x);
        self.ys.push(y);
    }

    pub fn set_coord(&mut self, idx: usize, y: f32) {
        self.ys[idx] = y;
    }
}

lazy_static! {
    static ref COORDS: MutStatic<Coordinates> = MutStatic::new();
}

#[wasm_bindgen]
pub fn new_coords() -> &Coordinates {
    COORDS.set(Coordinates::new());
}

#[wasm_bindgen]
pub fn add_coords(x: f32, y: f32) {
    COORDS.write().unwrap().add_coord(x, y);
}

#[wasm_bindgen]
pub fn set_coords(idx: usize, y: f32) {
    COORDS.write().unwrap().set_coord(idx, y);
}

#[wasm_bindgen]
pub fn print_all_coords() {
    for i in 0..COORDS.read().unwrap().xs.len() {
        let x = COORDS.read().unwrap().xs[i];
        let y = COORDS.read().unwrap().ys[i];
        print_coords(x, y);
    }
}
