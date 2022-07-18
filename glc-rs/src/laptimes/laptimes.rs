use lazy_static::lazy_static;
use mut_static::MutStatic;

trait ILaptimes {
    fn cold_track_empty_tank(&self) -> f32;
    fn cold_track_full_tank(&self) -> f32;
    fn warm_track_full_tank(&self) -> f32;
    fn cold_track_old_tyres(&self) -> f32;
}

struct Laptimes {
    cold_track_empty_tank_: f32,
    cold_track_full_tank_: f32,
    warm_track_full_tank_: f32,
    cold_track_old_tyres_: f32,
}

impl Laptimes {
    fn new() -> Self {
        Laptimes {
            cold_track_empty_tank_: 0.0,
            cold_track_full_tank_: 0.0,
            warm_track_full_tank_: 0.0,
            cold_track_old_tyres_: 0.0,
        }
    }

    pub fn set_cold_track_empty_tank(&mut self, laptime: f32) {
        self.cold_track_empty_tank_ = laptime;
    }

    pub fn set_warm_track_full_tank(&mut self, laptime: f32) {
        self.warm_track_full_tank_ = laptime;
    }

    pub fn set_cold_track_old_tyres(&mut self, laptime: f32) {
        self.cold_track_old_tyres_ = laptime;
    }

    pub fn set_cold_track_full_tank(&mut self, laptime: f32) {
        self.cold_track_full_tank_ = laptime;
    }
}

impl ILaptimes for Laptimes {
    fn cold_track_empty_tank(&self) -> f32 {
        return self.cold_track_empty_tank_;
    }

    fn cold_track_full_tank(&self) -> f32 {
        return self.cold_track_full_tank_;
    }

    fn warm_track_full_tank(&self) -> f32 {
        return self.warm_track_full_tank_;
    }

    fn cold_track_old_tyres(&self) -> f32 {
        return self.cold_track_old_tyres_;
    }
}

lazy_static! {
    static ref LAPTIMES: MutStatic<Laptimes> = MutStatic::new();
}

fn laptimes() -> &'static MutStatic<Laptimes> {
    if !LAPTIMES.is_set().expect("Error with MutStatic variable") {
        LAPTIMES
            .set(Laptimes::new())
            .expect("Could not create new MutStatic variable");
    }
    return &LAPTIMES;
}

pub fn set_laptime_cold_track_empty_tank(laptime: f32) {
    let mut laptimes = laptimes().write().unwrap();
    laptimes.set_cold_track_empty_tank(laptime);
}

pub fn set_laptime_cold_track_full_tank(laptime: f32) {
    let mut laptimes = laptimes().write().unwrap();
    laptimes.set_cold_track_full_tank(laptime);
}

pub fn set_laptime_warm_track_full_tank(laptime: f32) {
    let mut laptimes = laptimes().write().unwrap();
    laptimes.set_warm_track_full_tank(laptime);
}

pub fn set_laptime_cold_track_old_tyres(laptime: f32) {
    let mut laptimes = laptimes().write().unwrap();
    laptimes.set_cold_track_old_tyres(laptime);
}

pub fn laptime_cold_track_empty_tank() -> f32 {
    let laptimes = laptimes().read().unwrap();
    return laptimes.cold_track_empty_tank();
}

pub fn laptime_cold_track_full_tank() -> f32 {
    let laptimes = laptimes().read().unwrap();
    return laptimes.cold_track_full_tank();
}

pub fn laptime_warm_track_full_tank() -> f32 {
    let laptimes = laptimes().read().unwrap();
    return laptimes.warm_track_full_tank();
}

pub fn laptime_cold_track_old_tyres() -> f32 {
    let laptimes = laptimes().read().unwrap();
    return laptimes.cold_track_old_tyres();
}
