#![allow(dead_code)]
#![allow(unused_variables)]

mod timer;
mod sequencer;
mod lencounter;
mod pulse;
mod csv_rw;
use pulse::Pulse;

use cpal::{Sample, StreamConfig, SampleRate, BufferSize};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

fn main() {
    
    let clock = 44100_f32;
    let frequency = 440_f32;
    let gain = 0.01;
    let mut pulse= Pulse::new(frequency, clock);
    pulse.trigger();

    let host = cpal::default_host();
    let device = host.default_output_device().expect("no output device available");

    let sample_rate: cpal::SampleRate = SampleRate{0: 44100 as u32};
    let buffer_size: cpal::BufferSize = BufferSize::Default;
    let config: cpal::StreamConfig =  StreamConfig { channels : 1, sample_rate, buffer_size };
    
    let stream = device.build_output_stream(
        &config,
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            for sample in data.iter_mut() {
                pulse.tick();
                *sample = Sample::from(&(gain * pulse.get_value() as f32));
            } 
        },
        move |err| {},
    ).unwrap();
    stream.play().unwrap();

    loop {}

}
