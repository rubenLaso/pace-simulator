mod utils;

mod tyres;
use crate::tyres::wear;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn set_tyre_wear(idx: usize, perf: f32) {
    wear::set_perf(idx, perf);
}

#[wasm_bindgen]
pub fn add_tyre_wear(state: f32, perf: f32) {
    wear::add_perf(state, perf);
}

#[wasm_bindgen]
pub fn build_tyre_wear_spline() {
    wear::build_spline();
}

#[wasm_bindgen]
pub fn expected_laptime(tyre_state: f32) -> f32 {
    const DEFAULT_LAPTIME: f32 = 100.;
    let normalised_performance = wear::inv_performance(tyre_state);
    let laptime = DEFAULT_LAPTIME * normalised_performance;
    return laptime;
}

#[wasm_bindgen]
pub fn print_tyre_wear() {
    wear::print_tyre_wear_performance();
}

#[wasm_bindgen]
pub fn print_tyre_wear_chart() {
    wear::print_tyre_wear_chart_data();
}
