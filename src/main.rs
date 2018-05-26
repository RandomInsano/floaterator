/// Some documentation testing is going on here right now.

extern crate coreaudio;
extern crate piston_window;
extern crate rand;

mod generators;
mod controls;
mod filters;

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use piston_window::*;

use coreaudio::audio_unit::{AudioUnit, IOType, SampleFormat};
use coreaudio::audio_unit::render_callback::{self, data};


fn main() {
    run().unwrap()
}

fn run() -> Result<(), coreaudio::Error> {
    let mut freq = controls::Knob::new(440.0);
    let volume = controls::Knob::new(1.0);
    let sustain = controls::Knob::new_clamped(0.6, 0.0, 1.0);

    let generator = Arc::new(RwLock::new(generators::GenSaw::new(freq.clone())));
    let envelope = Arc::new(RwLock::new(filters::FilterADSR::new(generator, 200.0, 10.0, 0.6, 500.0)));
    let mut samples = filters::FilterVolume::new(envelope.clone(), volume.clone());





    // Construct an Output audio unit that delivers audio to the default output device.
    let mut audio_unit = try!(AudioUnit::new(IOType::DefaultOutput));

    let stream_format = try!(audio_unit.output_stream_format());
    println!("{:#?}", &stream_format);

    // For this example, our sine wave expects `f32` data.
    assert!(SampleFormat::F32 == stream_format.sample_format);

    type Args = render_callback::Args<data::NonInterleaved<f32>>;
    try!(audio_unit.set_render_callback(move |args| {
        let Args { num_frames, mut data, .. } = args;
        for i in 0..num_frames {
            let sample = samples.next().unwrap() as f32;
            for channel in data.channels_mut() {
                channel[i] = sample;
            }
        }
        Ok(())
    }));
    try!(audio_unit.start());



    let mut window: PistonWindow = WindowSettings::new(
        "Audio Thing",
        [300, 300]
    )
    .exit_on_esc(true)
    .build()
    .unwrap();

    let mut keys: HashMap<Key, f64> = HashMap::new();
    keys.insert(Key::A, 261.6); // C
    keys.insert(Key::Q, 277.2); // C#
    keys.insert(Key::S, 293.7); // D
    keys.insert(Key::W, 311.1); // D#
    keys.insert(Key::D, 329.6); // E
    keys.insert(Key::F, 349.2); // F
    keys.insert(Key::R, 370.0); // F#
    keys.insert(Key::G, 392.0); // G
    keys.insert(Key::T, 415.3); // G#
    keys.insert(Key::H, 440.0); // A
    keys.insert(Key::U, 466.2); // B#
    keys.insert(Key::J, 493.9); // B
    keys.insert(Key::K, 523.3); // C

    let mut events = Events::new(EventSettings::new().lazy(true));
    while let Some(e) = events.next(&mut window) {
        if let Some(Button::Keyboard(key)) = e.press_args() {
            //println!("Pressed keyboard key '{:?}'", key);
            if let Some(x) = keys.get(&key) {
                freq.write(*x);

                envelope.write().unwrap().press();
            }
        }
        if let Some(Button::Keyboard(_key)) = e.release_args() {
            //println!("Released keyboard key '{:?}'", key);

            envelope.write().unwrap().release();
        }        
    }

    Ok(())
}
