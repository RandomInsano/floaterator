use super::super::controls::Knob;

pub struct GenSine {
    value: f64,
    freq: Knob,
}
impl GenSine {
    pub fn new(freq: Knob) -> GenSine {
        GenSine {
            value: 0.0,
            freq: freq,
        }
    }

    fn get_freq(&self) -> f64 {
        self.freq.read()
    }
}
impl Iterator for GenSine {
    type Item = f64;

    fn next(&mut self) -> Option<f64> {
        self.value += self.freq.read() / 44_100.0;

        Some((self.value * 3.14159 * 2.0).sin())
    }
}