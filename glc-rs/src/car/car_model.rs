use lazy_static::lazy_static;
use mut_static::MutStatic;

use crate::utils::utils::log;
use crate::{console_log, laptimes};

pub const DEFAULT_TANK_CAPACITY: f32 = 75.0;
pub const DEFAULT_FUEL_PER_LAP: f32 = 2.75;
pub const DEFAULT_TIME_TO_FILL_TANK: f32 = 30.0;
pub const DEFAULT_TIME_TO_CHANGE_TYRES: f32 = 30.0;
pub const DEFAULT_TIME_TO_DRIVE_THROUGH: f32 = 30.0;

pub trait ICarModel {
    fn optimal_laptime(&self) -> f32;
    fn av_speed(&self) -> f32;
    fn fuel_tank_capacity(&self) -> f32;
    fn fuel_per_lap(&self) -> f32;
    fn time_to_fill_fuel_tank(&self) -> f32;
    fn time_to_change_tyres(&self) -> f32;
    fn time_to_drive_through(&self) -> f32;
    fn estimated_laps_to_go(&self, time_in_secs: f32) -> f32;
    fn secs_to_fill_fuel(&self, litres: f32) -> f32;
    fn secs_per_litre_refuel(&self) -> f32;
    fn litres_per_sec_refuel(&self) -> f32;
    fn litres_to_laps(&self, litre: f32) -> f32;
    fn laps_to_litres(&self, laps: u32) -> f32;
    fn fuel_performance_penalty(&self, litres: f32) -> f32;
}

pub struct CarModel {
    av_speed_: f32, // 100% of the track / laptime_cold_track

    fuel_tank_capacity_: f32,
    fuel_per_lap_: f32,

    time_to_fill_fuel_tank_: f32,
    time_to_change_tyres_: f32,

    time_to_drive_through_: f32,

    penalty_slope_: f32, // cached value of ((laptime_full_tank / laptime_empty_tank) - 1) / fuel_tank_capacity_
}

impl CarModel {
    pub fn new(
        fuel_tank_capacity: f32,
        fuel_per_lap: f32,
        time_to_fill_fuel_tank: f32,
        time_to_change_tyres: f32,
        time_to_drive_through: f32,
    ) -> Self {
        CarModel {
            av_speed_: 100.0 / laptimes::laptimes::laptime_cold_track_empty_tank(),
            fuel_tank_capacity_: fuel_tank_capacity,
            fuel_per_lap_: fuel_per_lap,
            time_to_fill_fuel_tank_: time_to_fill_fuel_tank,
            time_to_change_tyres_: time_to_change_tyres,
            time_to_drive_through_: time_to_drive_through,
            penalty_slope_: ((laptimes::laptimes::laptime_cold_track_full_tank()
                / laptimes::laptimes::laptime_cold_track_empty_tank())
                - 1.0)
                / fuel_tank_capacity,
        }
    }
}

impl ICarModel for CarModel {
    fn optimal_laptime(&self) -> f32 {
        return laptimes::laptimes::laptime_cold_track_empty_tank();
    }

    fn av_speed(&self) -> f32 {
        return self.av_speed_;
    }

    fn fuel_tank_capacity(&self) -> f32 {
        return self.fuel_tank_capacity_;
    }

    fn fuel_per_lap(&self) -> f32 {
        return self.fuel_per_lap_;
    }

    fn time_to_fill_fuel_tank(&self) -> f32 {
        return self.time_to_fill_fuel_tank_;
    }

    fn time_to_change_tyres(&self) -> f32 {
        return self.time_to_change_tyres_;
    }

    fn time_to_drive_through(&self) -> f32 {
        return self.time_to_drive_through_;
    }

    fn estimated_laps_to_go(&self, time_in_secs: f32) -> f32 {
        return time_in_secs / self.optimal_laptime();
    }

    fn secs_to_fill_fuel(&self, litres: f32) -> f32 {
        return litres * self.secs_per_litre_refuel();
    }

    fn secs_per_litre_refuel(&self) -> f32 {
        return self.time_to_fill_fuel_tank() / self.fuel_tank_capacity();
    }

    fn litres_per_sec_refuel(&self) -> f32 {
        return self.fuel_tank_capacity() / self.time_to_fill_fuel_tank();
    }

    fn litres_to_laps(&self, litres: f32) -> f32 {
        return (litres / self.fuel_per_lap()).floor();
    }

    fn laps_to_litres(&self, laps: u32) -> f32 {
        return (laps as f32) * self.fuel_per_lap();
    }

    fn fuel_performance_penalty(&self, litres: f32) -> f32 {
        return 1.0 + litres * self.penalty_slope_;
    }
}

struct CarModelData {
    fuel_tank_capacity_: f32,
    fuel_per_lap_: f32,
    time_to_fill_fuel_tank_: f32,
    time_to_change_tyres_: f32,
    time_to_drive_through_: f32,
}

impl CarModelData {
    pub fn new() -> Self {
        CarModelData {
            fuel_tank_capacity_: DEFAULT_TANK_CAPACITY,
            fuel_per_lap_: DEFAULT_FUEL_PER_LAP,
            time_to_fill_fuel_tank_: DEFAULT_TIME_TO_FILL_TANK,
            time_to_change_tyres_: DEFAULT_TIME_TO_CHANGE_TYRES,
            time_to_drive_through_: DEFAULT_TIME_TO_DRIVE_THROUGH,
        }
    }
}

lazy_static! {
    static ref CAR_MODEL: MutStatic<CarModel> = MutStatic::new();
    static ref CAR_MODEL_DATA: MutStatic<CarModelData> = MutStatic::new();
}

fn car_model_data() -> &'static MutStatic<CarModelData> {
    if !CAR_MODEL_DATA
        .is_set()
        .expect("Error with MutStatic variable")
    {
        CAR_MODEL_DATA
            .set(CarModelData::new())
            .expect("Could not create new MutStatic variable");
    }
    return &CAR_MODEL_DATA;
}

pub fn set_fuel_tank_capacity(tank: f32) {
    let mut car_model = car_model_data().write().unwrap();
    car_model.fuel_tank_capacity_ = tank;
}

pub fn set_fuel_per_lap(fuel: f32) {
    let mut car_model = car_model_data().write().unwrap();
    car_model.fuel_per_lap_ = fuel;
}

pub fn set_time_to_fill_fuel_tank(time: f32) {
    let mut car_model = car_model_data().write().unwrap();
    car_model.time_to_fill_fuel_tank_ = time;
}

pub fn set_time_to_change_tyres(time: f32) {
    let mut car_model = car_model_data().write().unwrap();
    car_model.time_to_change_tyres_ = time;
}

pub fn set_time_to_drive_through(time: f32) {
    let mut car_model = car_model_data().write().unwrap();
    car_model.time_to_drive_through_ = time;
}

pub fn build_model() {
    let car_model_data = car_model_data().read().unwrap();

    CAR_MODEL
        .set(CarModel::new(
            car_model_data.fuel_tank_capacity_,
            car_model_data.fuel_per_lap_,
            car_model_data.time_to_fill_fuel_tank_,
            car_model_data.time_to_change_tyres_,
            car_model_data.time_to_drive_through_,
        ))
        .expect("Could not build car model");

    let car_model = CAR_MODEL.read().unwrap();

    console_log!("Optimal laptime: {} s", car_model.optimal_laptime());
    console_log!("Av. speed: {} %/s", car_model.av_speed());
    console_log!("Fuel tank capacity: {} L", car_model.fuel_tank_capacity());
    console_log!("Fuel per lap: {} L", car_model.fuel_per_lap());
    console_log!(
        "Time to fill fuel tank: {} s",
        car_model.time_to_fill_fuel_tank()
    );
    console_log!(
        "Time to change tyres: {} s",
        car_model.time_to_change_tyres()
    );
    console_log!(
        "Time to drive through: {} s",
        car_model.time_to_drive_through()
    );
    // fn estimated_laps_to_go(&self, time_in_secs: f32) -> f32;
    // fn secs_to_fill_fuel(&self, litres: f32) -> f32;
    // fn secs_per_litre_refuel(&self) -> f32;
    // fn litres_per_sec_refuel(&self) -> f32;
    // fn litres_to_laps(&self, litre: f32) -> f32;
    // fn laps_to_litres(&self, laps: u32) -> f32;
    console_log!(
        "Fuel performance litre: +{} %/L",
        car_model.fuel_performance_penalty(1.0) - 1.0
    );
    // fn fuel_performance_penalty(&self, litres: f32) -> f32;
}
