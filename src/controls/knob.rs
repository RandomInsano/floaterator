/// Knobs are a linear control that can be plugged into various components. They're thread-safe
/// underneath so you can share them all over the place.

use std;
use std::sync::{
    Arc,
    RwLock
};

/// A Knob
pub struct Knob {
    value: Arc<RwLock<f64>>,
    max: Arc<RwLock<f64>>,
    min: Arc<RwLock<f64>>,
}

/// Allow making copies of Knobs. There are Arcs underneath so when sharing a
/// copied Knob, a change to one will change them all.
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
    /// Create a brand-new shiny Knob.
    pub fn new(value: f64) -> Self {
        Knob {
            value: Arc::new(RwLock::new(value)),
            max: Arc::new(RwLock::new(std::f64::MAX)),
            min: Arc::new(RwLock::new(std::f64::MIN)),
        }
    }

    /// Create a specific brand-new shiny Knob that won't allow values to be
    /// provided that are above `max` or below `min`.
    pub fn new_clamped(value: f64, max: f64, min: f64) -> Self {
        Knob {
            value: Arc::new(RwLock::new(value)),
            max: Arc::new(RwLock::new(max)),
            min: Arc::new(RwLock::new(min)),
        }
    }

    /// Get the current value of the Knob
    pub fn read(&self) -> f64 {
        *self.value.read().unwrap()
    }

    /// Set the current value. Note that if you enter a value outside
    /// of what `max` and `min` will allow, the input number will be silently
    /// clamped. There are also compile-time errors to save you from being
    /// silly in your code.
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