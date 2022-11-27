#![allow(dead_code)]
#![allow(unused_variables)]

mod audio;
mod csv_rw;
mod envelope;
mod lencounter;
mod pulse;
mod sequencer;
mod timer;
use cpal::traits::StreamTrait;
use pulse::Pulse;

fn main() {
    let clock = 44100_f32;
    let frequency = 440_f32;
    let mut pulse: Pulse = Pulse::new(frequency, clock);
    pulse.trigger();

    let stream = audio::stream(pulse);
    stream.play().unwrap();

    loop {}
}
