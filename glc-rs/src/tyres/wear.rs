use lazy_static::lazy_static;
use mut_static::MutStatic;

use splines::{Interpolation, Key, Spline};

use crate::utils::utils;

use wasm_bindgen::prelude::*;

struct WearChartData {
    xs_: Vec<f32>,
    ys_: Vec<f32>,
}

struct Wear {
    inv_performance_: [f32; 101],
}

impl WearChartData {
    pub fn new() -> Self {
        WearChartData {
            xs_: Vec::new(),
            ys_: Vec::new(),
        }
    }

    pub fn add_perf(&mut self, state: f32, performance: f32) {
        self.xs_.push(state);
        self.ys_.push(performance);
    }

    pub fn set_perf(&mut self, idx: usize, performance: f32) {
        self.ys_[idx] = performance;
    }
}

impl Wear {
    pub fn new() -> Self {
        Wear {
            inv_performance_: [1.0; 101],
        }
    }

    pub fn build_spline(&mut self, chart_data: &WearChartData) {
        let mut data: Vec<Key<f32, f32>> = Vec::new();

        for i in 0..chart_data.xs_.len() {
            let x = chart_data.xs_[i] as f32;
            let y = chart_data.ys_[i] as f32;

            data.push(Key::new(x, y, Interpolation::Cosine));
        }

        let spline = Spline::from_vec(data);

        for i in 0..self.inv_performance_.len() {
            let x = i as f32;
            let y = spline.clamped_sample(x).unwrap();
            self.inv_performance_[i] = 100.0 / y;
        }
    }

    pub fn perf_penalty(&mut self, state: f32) -> f32 {
        let idx: usize = state as usize;
        return self.inv_performance_[idx];
    }
}

lazy_static! {
    static ref WEAR_CHART_DATA: MutStatic<WearChartData> = MutStatic::new();
    static ref WEAR: MutStatic<Wear> = MutStatic::new();
}

fn wear_chart_data() -> &'static MutStatic<WearChartData> {
    if !WEAR_CHART_DATA
        .is_set()
        .expect("Error with MutStatic variable")
    {
        WEAR_CHART_DATA
            .set(WearChartData::new())
            .expect("Could not create new MutStatic variable");
    }
    return &WEAR_CHART_DATA;
}

fn wear() -> &'static MutStatic<Wear> {
    if !WEAR.is_set().expect("Error with MutStatic variable") {
        WEAR.set(Wear::new())
            .expect("Could not create new MutStatic variable");
    }
    return &WEAR;
}

#[wasm_bindgen]
pub fn add_perf(state: f32, performance: f32) {
    let mut chart_data = wear_chart_data().write().unwrap();
    chart_data.add_perf(state, performance);
}

#[wasm_bindgen]
pub fn set_perf(idx: usize, performance: f32) {
    let mut chart_data = wear_chart_data().write().unwrap();
    chart_data.set_perf(idx, performance);
}

#[wasm_bindgen]
pub fn build_spline() {
    let chart_data = wear_chart_data().read().unwrap();
    let mut wear = wear().write().unwrap();
    wear.build_spline(chart_data.as_ref());
}

#[wasm_bindgen]
pub fn inv_performance(tyre_state: f32) -> f32 {
    let mut wear = wear().write().unwrap();
    return wear.perf_penalty(tyre_state);
}

#[wasm_bindgen]
pub fn print_tyre_wear_performance() {
    let wear = wear().read().unwrap();

    for i in 0..wear.inv_performance_.len() {
        let x = i as f32;
        let y = wear.inv_performance_[i];
        utils::print_coords(x, y);
    }
}

#[wasm_bindgen]
pub fn print_tyre_wear_chart_data() {
    let wear = wear_chart_data().read().unwrap();

    for i in 0..wear.xs_.len() {
        let x = wear.xs_[i] as f32;
        let y = wear.ys_[i] as f32;
        utils::print_coords(x, y);
    }
}
