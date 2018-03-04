use std::sync::{Arc, RwLock};

pub struct Knob {
    value: Arc<RwLock<f64>>,
}
// Not sure why I wasn't able to derive Copy
impl Clone for Knob {
    fn clone(&self) -> Self {
        Knob {
            value: self.value.clone(),
        }
    }
}
impl Knob {
    pub fn new(value: f64) -> Self {
        Knob {
            value: Arc::new(RwLock::new(value)),
        }
    }
    pub fn read(&self) -> f64 {
        *self.value.read().unwrap()
    }
    pub fn write(&mut self, value: f64) {
        *self.value.write().unwrap() = value;
    }
}