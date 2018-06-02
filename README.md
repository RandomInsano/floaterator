# Sound Floaterator

I was inspired by some [eurorack synth gear](https://www.modulargrid.net/) to build my own software-defined synth. Because sound libraries usually just want a big old stream of floating point numbers to drive a speaker, that's all this library does. It's a series of 64bit floating-point iterators that you can chain together. Many of them are controlled with virtual knobs and switches.

So far, there are these lovely features:

1. A broken noise generator (only one frequency)
2. A Sine Wave generator
3. A volume filter
4. An [ADSR envelope](http://en.wikiaudio.org/ADSR_envelope) filter
5. Control knobs that you can connect in at creation time

Here's a usage example:

```rust
    let mut freq = controls::Knob::new(440.0);
    let volume = controls::Knob::new(1.0);

    let generator = Arc::new(RwLock::new(generators::GenSine::new(freq.clone())));
    let envelope = Arc::new(RwLock::new(filters::FilterADSR::new(generator, 200.0, 10.0, 0.6, 100.0)));
    let mut samples = filters::FilterVolume::new(envelope.clone(), volume.clone());

    // TODO: Hide the thread safety away where nobody has to see it
    // TODO: Moar knobs! (Specifically on the ADSR filter)
```

Currently, I'm piping this stuff through CoreAudio and have a disgusting shim for keyboard entry using Piston to give me a single octave musical keyboard. That'll change eventually, but I'm putting this project on hold until the next [SkullSpace](http://skull.space) synth jam.

Just because I'm not iterating on it doesn't mean I won't accept pull requests though! If you want to contribute, maybe file an issue first so we can discuss coding attack plans.
