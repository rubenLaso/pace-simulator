pub struct F32 {
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
