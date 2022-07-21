use crate::laptimes::laptimes::*;

pub trait IPerfModel {
    fn performance_penalty(&self, temp: f32) -> f32;
}

pub struct PerfModel {
    max_temp_: f32,
    min_temp_: f32,
    laptime_cold_: f32,
    laptime_warm_: f32,
}

impl PerfModel {
    pub fn new(max_temp: f32, min_temp: f32, laptime_cold: f32, laptime_warm: f32) -> Self {
        PerfModel {
            max_temp_: max_temp,
            min_temp_: min_temp,
            laptime_cold_: laptime_cold,
            laptime_warm_: laptime_warm,
        }
    }
}

impl IPerfModel for PerfModel {
    fn performance_penalty(&self, temp: f32) -> f32 {
        let slope =
            (self.laptime_warm_ / self.laptime_cold_ - 1.0) / (self.max_temp_ - self.min_temp_);
        let intercept = 1.0 - slope * self.min_temp_;

        return intercept + temp * slope;
    }
}

pub fn performance_penalty(temp: f32, max_temp: f32, min_temp: f32) -> f32 {
    let laptime_cold = laptime_cold_track_full_tank();
    let laptime_warm = laptime_warm_track_full_tank();

    let perf_model: PerfModel = PerfModel::new(max_temp, min_temp, laptime_cold, laptime_warm);

    return perf_model.performance_penalty(temp);
}
