use super::super::controls::Knob;

pub struct GenSaw {
    value: f64,
    freq: Knob,
}

impl GenSaw {
    pub fn new(freq: Knob) -> Box<Self> {
        let a = Self {
            value: 0.0,
            freq: freq,
        };

        Box::new(a)
    }
}

impl Iterator for GenSaw {
    type Item = f64;

    fn next(&mut self) -> Option<f64> {
        self.value += self.freq.read() / 44_100.0;

        Some((self.value % 1.0) - 0.5)
    }
}