mod utils;

mod car;
mod laptimes;
mod track;
mod tyres;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn simulate() {
    car::car_model::build_model();
}

#[wasm_bindgen]
pub fn default_tank_capacity() -> f32 {
    return car::car_model::DEFAULT_TANK_CAPACITY;
}

#[wasm_bindgen]
pub fn default_fuel_per_lap() -> f32 {
    return car::car_model::DEFAULT_FUEL_PER_LAP;
}

#[wasm_bindgen]
pub fn default_time_to_fill_tank() -> f32 {
    return car::car_model::DEFAULT_TIME_TO_FILL_TANK;
}

#[wasm_bindgen]
pub fn default_time_to_change_tyres() -> f32 {
    return car::car_model::DEFAULT_TIME_TO_CHANGE_TYRES;
}

#[wasm_bindgen]
pub fn default_time_to_drive_through() -> f32 {
    return car::car_model::DEFAULT_TIME_TO_DRIVE_THROUGH;
}

#[wasm_bindgen]
pub fn set_fuel_tank_capacity(tank: f32) {
    car::car_model::set_fuel_tank_capacity(tank);
}

#[wasm_bindgen]
pub fn set_fuel_per_lap(fuel: f32) {
    car::car_model::set_fuel_per_lap(fuel);
}

#[wasm_bindgen]
pub fn set_time_to_fill_fuel_tank(time: f32) {
    car::car_model::set_time_to_fill_fuel_tank(time);
}

#[wasm_bindgen]
pub fn set_time_to_change_tyres(time: f32) {
    car::car_model::set_time_to_change_tyres(time);
}

#[wasm_bindgen]
pub fn set_time_to_drive_through(time: f32) {
    car::car_model::set_time_to_drive_through(time);
}

#[wasm_bindgen]
pub fn default_laptime_cold_track_empty_tank() -> f32 {
    return laptimes::laptimes::LAPTIME_COLD_TRACK_EMPTY_TANK;
}

#[wasm_bindgen]
pub fn default_laptime_cold_track_full_tank() -> f32 {
    return laptimes::laptimes::LAPTIME_COLD_TRACK_FULL_TANK;
}

#[wasm_bindgen]
pub fn default_laptime_warm_track_full_tank() -> f32 {
    return laptimes::laptimes::LAPTIME_WARM_TRACK_FULL_TANK;
}

#[wasm_bindgen]
pub fn default_laptime_cold_track_old_tyres() -> f32 {
    return laptimes::laptimes::LAPTIME_COLD_TRACK_OLD_TYRES;
}

#[wasm_bindgen]
pub fn set_laptime_cold_track_full_tank(laptime: f32) {
    laptimes::laptimes::set_laptime_cold_track_full_tank(laptime);
}

#[wasm_bindgen]
pub fn set_laptime_cold_track_empty_tank(laptime: f32) {
    laptimes::laptimes::set_laptime_cold_track_empty_tank(laptime);
}

#[wasm_bindgen]
pub fn set_laptime_warm_track_full_tank(laptime: f32) {
    laptimes::laptimes::set_laptime_warm_track_full_tank(laptime);
}

#[wasm_bindgen]
pub fn set_laptime_cold_track_old_tyres(laptime: f32) {
    laptimes::laptimes::set_laptime_cold_track_old_tyres(laptime);
}

#[wasm_bindgen]
pub fn get_laptime_cold_track_empty_tank() -> f32 {
    return laptimes::laptimes::laptime_cold_track_empty_tank();
}

#[wasm_bindgen]
pub fn get_laptime_cold_track_full_tank() -> f32 {
    return laptimes::laptimes::laptime_cold_track_full_tank();
}

#[wasm_bindgen]
pub fn get_laptime_warm_track_full_tank() -> f32 {
    return laptimes::laptimes::laptime_warm_track_full_tank();
}

#[wasm_bindgen]
pub fn get_laptime_cold_track_old_tyres() -> f32 {
    return laptimes::laptimes::laptime_cold_track_old_tyres();
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

    let laptime = get_laptime_cold_track_full_tank() * track_temp_inv_perf;

    return laptime;
}
