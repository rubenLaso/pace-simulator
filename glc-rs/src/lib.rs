mod utils;

mod track;
mod tyres;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn set_laptime_cold_track(laptime: f32) {
    track::perf::set_laptime_cold(laptime);
}

#[wasm_bindgen]
pub fn set_laptime_warm_track(laptime: f32) {
    track::perf::set_laptime_warm(laptime);
}

#[wasm_bindgen]
pub fn get_laptime_cold_track() -> f32 {
    return track::perf::get_laptime_cold();
}

#[wasm_bindgen]
pub fn get_laptime_warm_track() -> f32 {
    return track::perf::get_laptime_warm();
}

#[wasm_bindgen]
pub fn set_track_temp(idx: usize, temp: f32) {
    track::temp::set_temp(idx, temp);
}

#[wasm_bindgen]
pub fn add_track_temp(time: f32, temp: f32) {
    track::temp::add_temp(time, temp);
}

#[wasm_bindgen]
pub fn build_track_temp_spline() {
    track::temp::build_spline();
}

#[wasm_bindgen]
pub fn get_max_track_temp() -> f32 {
    return track::temp::max_temp();
}

#[wasm_bindgen]
pub fn get_min_track_temp() -> f32 {
    return track::temp::min_temp();
}

#[wasm_bindgen]
pub fn print_track_temp() {
    track::temp::print_track_temp_performance();
}

#[wasm_bindgen]
pub fn print_track_temp_chart() {
    track::temp::print_track_temp_chart_data();
}

#[wasm_bindgen]
pub fn set_tyre_wear(idx: usize, perf: f32) {
    tyres::wear::set_perf(idx, perf);
}

#[wasm_bindgen]
pub fn add_tyre_wear(state: f32, perf: f32) {
    tyres::wear::add_perf(state, perf);
}

#[wasm_bindgen]
pub fn build_tyre_wear_spline() {
    tyres::wear::build_spline();
}

#[wasm_bindgen]
pub fn print_tyre_wear() {
    tyres::wear::print_tyre_wear_performance();
}

#[wasm_bindgen]
pub fn print_tyre_wear_chart() {
    tyres::wear::print_tyre_wear_chart_data();
}

#[wasm_bindgen]
pub fn expected_laptime_by_tyre_wear(base_laptime: f32, tyre_state: f32) -> f32 {
    let tyre_inv_perf = tyres::wear::inv_performance(tyre_state);
    let laptime = base_laptime * tyre_inv_perf;

    return laptime;
}

#[wasm_bindgen]
pub fn expected_laptime_by_track_temp(track_temp: f32) -> f32 {
    let track_temp_inv_perf = track::perf::performance_penalty(
        track_temp,
        track::temp::max_temp(),
        track::temp::min_temp(),
    );

    let laptime = track::perf::get_laptime_cold() * track_temp_inv_perf;

    return laptime;
}
