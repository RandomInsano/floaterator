/// Some documentation testing is going on here right now.

extern crate cpal;
extern crate rand;
extern crate pscontroller_rs;
extern crate futures;

mod generators;
mod controls;
mod filters;

use std::{
    collections::HashMap,
    thread,
    sync::{
        Arc,
        Mutex,
        RwLock
    }
};
use futures::{
    channel::mpsc,
    Async,
    Future,
    Sink,
    Stream
};
use cpal::{
    EventLoop,
    StreamData,
    UnknownTypeOutputBuffer
};


fn main() {
    let mut freq = controls::Knob::new(440.0);
    let volume = controls::Knob::new(1.0);
    let sustain = controls::Knob::new_clamped(0.6, 0.0, 1.0);
    let play_note = controls::Switch::new();

    let generator = generators::GenSaw::new(freq.clone());
    let mut envelope = filters::FilterADSR::new(generator, play_note.clone(), 200.0, 10.0, 0.6, 500.0);
    let mut samples = filters::FilterVolume::new(envelope, volume.clone());

    let samples = Arc::new(RwLock::new(samples));

    let (tx_sound, rx_sound) = mpsc::channel(1000);

    // Cpal goodies (stolen straight from the example docs. :D)
    let event_loop = EventLoop::new();
    let device = cpal::default_output_device()
        .expect("no output device available");
    let format = device.default_output_format()
        .expect("error while querying formats");
    let stream = event_loop
        .build_output_stream(&device, &format)
        .unwrap();

    let mut forever = std::iter::repeat(3.0 as f32);

    thread::spawn(move|| {
        loop {
            tx_sound.send(forever.next()).wait();
        }
    });

    event_loop.play_stream(stream);

    event_loop.run(move |_stream_id, data| {
        match data {
            cpal::StreamData::Output { buffer: cpal::UnknownTypeOutputBuffer::F32(mut buffer) } => {
                for sample in buffer.chunks_mut(format.channels as usize) {
                    for out in sample.iter_mut() {
                        *out = rx_sound.try_next().unwrap().expect("?!")
                    }
                }
            },
            _ => (),
        }
    });


    /*
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


    freq.write(*x);
    play_note.set(true);
    play_note.set(false);
    */


}
