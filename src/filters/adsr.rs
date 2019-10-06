type FloatStream = Box<Iterator<Item=f64>>;

use controls::Switch;

#[derive(Debug)]
enum ADSRState {
    Start,
    Attack,
    Decay,
    Sustain,
    Release,
    Idle,
}
/// FilterADSR
///
/// The attack, decay, sustain, release envelope helps us approximate real
/// instruments and how the vibrations they create sound over time. Really,
/// we're controlling volume over some duration and so a lot of the internal
/// code here is going to be line drawing with Y being the volume and X
/// being how long the part of the note should ring in or out for
pub struct FilterADSR {
    state: ADSRState,
    /// When this switch is turned on, play a note
    play: Switch,
    /// This is essentially our X for line drawing. Always starts at zero.
    counter: f64,
    /// This is our M for line drawing
    current_slope: f64,
    /// This is our Y offset
    current_offset: f64,
    /// Whether the note can be released or not
    allow_release: bool,

    generator: FloatStream,
    attack: f64,
    decay: f64,
    sustain: f64,
    release: f64,
}

impl FilterADSR {
    fn calc_slope(startx: f64, starty: f64, endx: f64, endy: f64) -> f64 {
        if endx == startx {
            return endy - starty;
        }

        (endy - starty) / (endx - startx)
    }

    pub fn new(generator: FloatStream, play: Switch, a: f64, d: f64, s: f64, r: f64) -> Box<FilterADSR> {
        let a = FilterADSR {
            state: ADSRState::Idle,
            play: play,
            counter: 0.0,
            current_slope: 0.0,
            current_offset: 0.0,
            allow_release: false,

            generator: generator,

            //TODO: Create interface for generator(s) and pull the sample rate from them
            attack: a * 44.1,
            decay: d * 44.1,
            sustain: s,
            release: r * 44.1,
        };

        Box::new(a)
    }
}
impl Iterator for FilterADSR {
    type Item = f64;

    fn next(&mut self) -> Option<f64> {
        match self.state {
            ADSRState::Idle => {
                if self.play.get() {
                    self.state = ADSRState::Start;
                }

                return Some(0.0)
            },
            _ => {
                if !self.play.get() {
                    self.allow_release = true;
                }
            }
        }

        if let Some(x) = self.generator.next() {
            match self.state {
                ADSRState::Start => {
                    self.state = ADSRState::Attack;
                    self.counter = 0.0;
                    self.current_slope = FilterADSR::calc_slope(0.0, 0.0, self.attack, 1.0);
                    self.current_offset = 0.0;
                    self.allow_release = false;
                },
                ADSRState::Attack => {
                    if self.counter > self.attack {
                        self.state = ADSRState::Decay;
                        self.counter = 0.0;
                        self.current_slope = FilterADSR::calc_slope(0.0, 1.0, self.decay, self.sustain);
                        self.current_offset = 1.0;
                    } else {
                        self.counter += 1.0;
                    }
                },
                ADSRState::Decay => {
                    if self.counter > self.decay {
                        self.state = ADSRState::Sustain;
                    } else {
                        self.counter += 1.0;
                    }                    
                }
                ADSRState::Sustain => {
                    if self.allow_release {
                        self.state = ADSRState::Release;

                        self.counter = 0.0;
                        self.current_slope = FilterADSR::calc_slope(0.0, self.sustain, self.release, 0.0);
                        self.current_offset = self.sustain;
                    }
                }
                ADSRState::Release => {
                    if self.counter > self.release {
                        self.state = ADSRState::Idle;
                    } else {
                        self.counter += 1.0;
                    }
                }                
                ADSRState::Idle => return Some(0.0)
            }

            let volume = self.counter * self.current_slope + self.current_offset;
            return Some(volume * x);
        }

        None
    }
}
