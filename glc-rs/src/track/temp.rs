use lazy_static::lazy_static;
use mut_static::MutStatic;

use splines::{Interpolation, Key, Spline};

use crate::utils::utils;

pub struct TempChartData {
    xs_: Vec<f32>,
    ys_: Vec<f32>,
}

impl TempChartData {
    pub fn new() -> Self {
        TempChartData {
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

pub trait ITempModel {
    fn temperature(&self, time_in_secs: f32) -> f32;
    fn max_temp(&self) -> f32;
    fn min_temp(&self) -> f32;
}

pub struct TempModel {
    temperatures_: [f32; 24 * 60],
    max_temp_: f32,
    min_temp_: f32,
}

impl TempModel {
    pub fn new() -> Self {
        TempModel {
            temperatures_: [1.0; 24 * 60],
            max_temp_: 0.0,
            min_temp_: 0.0,
        }
    }

    pub fn build_spline(&mut self, chart_data: &TempChartData) {
        let mut data: Vec<Key<f32, f32>> = Vec::new();

        for i in 0..chart_data.xs_.len() {
            let x = chart_data.xs_[i] as f32;
            let t = x * 60.0;
            let y = chart_data.ys_[i] as f32;
            data.push(Key::new(t, y, Interpolation::Linear));
        }
        // Temp at 24h == 0h
        data.push(Key::new(
            24.0 * 60.0,
            chart_data.ys_[0],
            Interpolation::Linear,
        ));

        let spline = Spline::from_vec(data);

        for i in 0..self.temperatures_.len() {
            let t = i as f32;
            let y = spline.clamped_sample(t).unwrap();
            self.temperatures_[i] = y;
        }

        self.max_temp_ = self.temperatures_.iter().fold(f32::MIN, |a, &b| a.max(b));

        self.min_temp_ = self.temperatures_.iter().fold(f32::MAX, |a, &b| a.min(b));
    }
}

impl ITempModel for TempModel {
    fn temperature(&self, time_in_secs: f32) -> f32 {
        let time_in_mins = time_in_secs / 60.0;
        return self.temperatures_[time_in_mins as usize];
    }

    fn max_temp(&self) -> f32 {
        return self.max_temp_;
    }

    fn min_temp(&self) -> f32 {
        return self.min_temp_;
    }
}

lazy_static! {
    static ref TEMP_CHART_DATA: MutStatic<TempChartData> = MutStatic::new();
    static ref TEMP: MutStatic<TempModel> = MutStatic::new();
}

fn temp_chart_data() -> &'static MutStatic<TempChartData> {
    if !TEMP_CHART_DATA
        .is_set()
        .expect("Error with MutStatic variable")
    {
        TEMP_CHART_DATA
            .set(TempChartData::new())
            .expect("Could not create new MutStatic variable");
    }
    return &TEMP_CHART_DATA;
}

fn temp() -> &'static MutStatic<TempModel> {
    if !TEMP.is_set().expect("Error with MutStatic variable") {
        TEMP.set(TempModel::new())
            .expect("Could not create new MutStatic variable");
    }
    return &TEMP;
}

pub fn add_temp(time: f32, temperature: f32) {
    let mut chart_data = temp_chart_data().write().unwrap();
    chart_data.add_perf(time, temperature);
}

pub fn set_temp(idx: usize, temperature: f32) {
    let mut chart_data = temp_chart_data().write().unwrap();
    chart_data.set_perf(idx, temperature);
}

pub fn build_spline() {
    let chart_data = temp_chart_data().read().unwrap();
    let mut temp = temp().write().unwrap();
    temp.build_spline(chart_data.as_ref());
}

pub fn max_temp() -> f32 {
    let temp = temp().read().unwrap();
    return temp.max_temp();
}

pub fn min_temp() -> f32 {
    let temp = temp().read().unwrap();
    return temp.min_temp();
}

pub fn print_track_temp_performance() {
    let temp = temp().read().unwrap();

    for i in 0..temp.temperatures_.len() {
        let x = i as f32;
        let y = temp.temperatures_[i];
        utils::print_coords(x, y);
    }
}

pub fn print_track_temp_chart_data() {
    let temp = temp_chart_data().read().unwrap();

    for i in 0..temp.xs_.len() {
        let x = temp.xs_[i] as f32;
        let y = temp.ys_[i] as f32;
        utils::print_coords(x, y);
    }
}
