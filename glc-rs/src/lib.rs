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
        utils::print_coords(p.x as f32, p.y as f32);
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
        utils::print_coords(point.x as f32, point.y as f32);
    }
}

#[wasm_bindgen]
pub fn expected_laptime(tyre_performance: f32) -> f32 {
    const DEFAULT_LAPTIME: f32 = 100.;
    let normalised_performance = tyre_performance / 100.0;
    let laptime = DEFAULT_LAPTIME / normalised_performance;
    laptime
}
