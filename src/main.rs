#![allow(dead_code)]
#![allow(unused_variables)]

mod csv_rw;
mod lencounter;
mod pulse;
mod sequencer;
mod timer;
mod envelope;
use pulse::Pulse;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{BufferSize, Sample, SampleRate, StreamConfig};

fn main() {
    let clock = 44100_f32;
    let frequency = 440_f32;
    let gain = 0.05;
    let mut pulse = Pulse::new(frequency, clock);
    pulse.trigger();

    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("no output device available");

    let sample_rate = SampleRate { 0: 44100 as u32 };
    let buffer_size = BufferSize::Default;
    let channels = 1;
    let config = StreamConfig {
        channels,
        sample_rate,
        buffer_size,
    };

    let stream = device
        .build_output_stream(
            &config,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                for sample in data.iter_mut() {
                    pulse.tick();
                    let value = gain * (pulse.get_value() as f32)/16.;
                    if value.abs() > 1. {
                        panic!("Amplitude out of bounds.")
                    }
                    *sample = Sample::from(&value);
                }
            },
            move |err| {},
        )
        .unwrap();
    stream.play().unwrap();

    loop {}
}
