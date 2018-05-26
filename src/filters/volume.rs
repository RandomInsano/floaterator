use super::super::controls::Knob;

type FloatStream = Box<Iterator<Item=f64>>;

pub struct FilterVolume {
    generator: FloatStream,
    value: Knob,
}

impl FilterVolume {
    pub fn new(generator: FloatStream, volume: Knob) -> FilterVolume {
        FilterVolume {
            generator: generator,
            value: volume,
        }
    }
}

impl Iterator for FilterVolume {
    type Item = f64;

    fn next(&mut self) -> Option<f64> {
        if let Some(x) = self.generator.next() {
            return Some(x * self.value.read())
        }

        None
    }
}