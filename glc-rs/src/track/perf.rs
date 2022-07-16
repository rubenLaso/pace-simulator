use lazy_static::lazy_static;
use mut_static::MutStatic;

struct F32 {
    value_: f32,
}

impl F32 {
    pub fn new(value: f32) -> Self {
        F32 { value_: value }
    }

    pub fn get(&self) -> f32 {
        return self.value_;
    }

    pub fn set(&mut self, value: f32) {
        return self.value_ = value;
    }
}

lazy_static! {
    static ref LAPTIME_COLD: MutStatic<F32> = MutStatic::new();
    static ref LAPTIME_WARM: MutStatic<F32> = MutStatic::new();
    static ref MIN_TEMP: MutStatic<F32> = MutStatic::new();
    static ref MAX_TEMP: MutStatic<F32> = MutStatic::new();
}

fn laptime_cold() -> &'static MutStatic<F32> {
    if !LAPTIME_COLD
        .is_set()
        .expect("Error with MutStatic variable")
    {
        LAPTIME_COLD
            .set(F32::new(0.0))
            .expect("Could not create new MutStatic variable");
    }
    return &LAPTIME_COLD;
}

fn laptime_warm() -> &'static MutStatic<F32> {
    if !LAPTIME_WARM
        .is_set()
        .expect("Error with MutStatic variable")
    {
        LAPTIME_WARM
            .set(F32::new(0.0))
            .expect("Could not create new MutStatic variable");
    }
    return &LAPTIME_WARM;
}

pub fn set_laptime_cold(laptime: f32) {
    let mut laptime_cold = laptime_cold().write().unwrap();
    laptime_cold.set(laptime);
}

pub fn set_laptime_warm(laptime: f32) {
    let mut laptime_warm = laptime_warm().write().unwrap();
    laptime_warm.set(laptime);
}

pub fn get_laptime_cold() -> f32 {
    let laptime = laptime_cold().read().unwrap();
    return laptime.get();
}

pub fn get_laptime_warm() -> f32 {
    let laptime = laptime_warm().read().unwrap();
    return laptime.get();
}

pub fn performance_penalty(temp: f32, max_temp: f32, min_temp: f32) -> f32 {
    let laptime_cold = get_laptime_cold();
    let laptime_warm = get_laptime_warm();

    let slope = (laptime_warm / laptime_cold - 1.0) / (max_temp - min_temp);
    let intercept = 1.0 - slope * min_temp;

    return intercept + temp * slope;
}
