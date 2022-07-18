use crate::laptimes::laptimes::*;

pub fn performance_penalty(temp: f32, max_temp: f32, min_temp: f32) -> f32 {
    let laptime_cold = laptime_cold_track_full_tank();
    let laptime_warm = laptime_warm_track_full_tank();

    let slope = (laptime_warm / laptime_cold - 1.0) / (max_temp - min_temp);
    let intercept = 1.0 - slope * min_temp;

    return intercept + temp * slope;
}
