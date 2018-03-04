use std;
use std::sync::{Arc, RwLock};

pub struct Knob {
    value: Arc<RwLock<f64>>,
    max: Arc<RwLock<f64>>,
    min: Arc<RwLock<f64>>,
}

impl Clone for Knob {
    fn clone(&self) -> Self {
        Knob {
            value: self.value.clone(),
            max: self.value.clone(),
            min: self.value.clone(),
        }
    }
}

impl Knob {
    pub fn new(value: f64) -> Self {
        Knob {
            value: Arc::new(RwLock::new(value)),
            max: Arc::new(RwLock::new(std::f64::MAX)),
            min: Arc::new(RwLock::new(std::f64::MIN)),
        }
    }

    pub fn new_clamped(value: f64, max: f64, min: f64) -> Self {
        Knob {
            value: Arc::new(RwLock::new(value)),
            max: Arc::new(RwLock::new(max)),
            min: Arc::new(RwLock::new(min)),
        }
    }

    pub fn read(&self) -> f64 {
        *self.value.read().unwrap()
    }

    pub fn write(&mut self, mut value: f64) {
        let max = self.get_max();
        let min = self.get_min();

        // Put in some compile-time errors if I'm doing something dumb
        assert!(value < max);
        assert!(value > min);

        if value > max {
            value = max;
        } else if value < min {
            value = min;
        }

        *self.value.write().unwrap() = value;
    }

    pub fn get_max(&mut self) -> f64 {
        *self.max.read().unwrap()
    }

    pub fn set_max(&mut self, value: f64) {
        *self.max.write().unwrap() = value;
    }

    pub fn get_min(&mut self) -> f64 {
        *self.min.read().unwrap()
    }

    pub fn set_min(&mut self, value: f64) {
        *self.min.write().unwrap() = value;
    }
}