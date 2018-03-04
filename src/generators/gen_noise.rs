use std::sync::{Arc, RwLock};

use rand::Rng;

type FloatStream = Arc<RwLock<Iterator<Item=f64>>>;

struct GenNoise {
    value: f64,
    freq: f64,
    rate: f64,
}

#[allow(dead_code)]
impl GenNoise {
    fn new(freq: f64) -> Self {
        Self {
            value: 0.0,
            freq: freq,
            rate: 44100.0,
        }
    }

    pub fn get_rate(&self) -> f64 {
        self.rate
    }
}

impl Iterator for GenSquare {
    type Item = f64;

    fn next(&mut self) -> Option<f64> {
        //self.value += self.freq / 44_100.0;

        let value = rand::thread_rng().gen::<f64>();

        Some(value)
    }
}
