use super::perf::IPerfModel;
use super::temp::ITempModel;

pub trait ITrack {
    fn time(&self) -> f32;
    fn temperature(&self) -> f32;
    fn temperature_at(&self, time_in_secs: f32) -> f32;
    fn advance_time(&mut self, timestep: f32);
    fn laptime_penalty(&self) -> f32;
}

pub struct Track {
    time_: f32,

    temp_model_: Box<dyn ITempModel>,
    perf_model_: Box<dyn IPerfModel>,

    current_temp_: f32,         // Cached value from temp_model.temperature(time_)
    current_perf_penalty_: f32, // Cached value from perf_model.performance(current_temp_)
}

impl Track {
    pub fn new(
        start_time: f32,
        temp_model: Box<dyn ITempModel>,
        perf_model: Box<dyn IPerfModel>,
    ) -> Self {
        let mut track = Track {
            time_: start_time,
            temp_model_: temp_model,
            perf_model_: perf_model,
            current_temp_: 0.0,         // temp_model.temperature(time_),
            current_perf_penalty_: 1.0, // perf_model.performance_penalty(current_temp_),
        };
        track.current_temp_ = track.temp_model_.temperature(track.time_);
        track.current_perf_penalty_ = track.perf_model_.performance_penalty(track.current_temp_);

        return track;
    }
}

impl ITrack for Track {
    fn time(&self) -> f32 {
        return self.time_;
    }

    fn temperature(&self) -> f32 {
        return self.current_temp_;
    }

    fn temperature_at(&self, time_in_secs: f32) -> f32 {
        return self.temp_model_.temperature(time_in_secs);
    }

    fn advance_time(&mut self, timestep: f32) {
        self.time_ += timestep;
        self.current_temp_ = self.temp_model_.temperature(self.time_);
        self.current_perf_penalty_ = self.perf_model_.performance_penalty(self.current_temp_);
    }

    fn laptime_penalty(&self) -> f32 {
        return self.current_perf_penalty_;
    }
}
