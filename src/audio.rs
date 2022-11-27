use cpal::traits::{DeviceTrait, HostTrait};
use cpal::{BufferSize, Device, Sample, SampleRate, Stream, StreamConfig};

use crate::pulse::Pulse;

/// Setup default device for audio stream
fn setup_device() -> Device {
    cpal::default_host()
        .default_output_device()
        .expect("no output device available")
}

/// Setup stream configuration
fn setup_stream_config() -> StreamConfig {
    StreamConfig {
        channels: 1,
        sample_rate: SampleRate(44100_u32),
        buffer_size: BufferSize::Default,
    }
}

/// Build stream object
pub fn stream(mut pulse: Pulse) -> Stream {
    let device = setup_device();
    let config = setup_stream_config();
    let stream = device
        .build_output_stream(
            &config,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                for sample in data.iter_mut() {
                    pulse.tick();
                    let value: f32 = 0.05 * (pulse.get_value() as f32) / 16.;
                    if value.abs() > 1. {
                        panic!("Amplitude out of bounds.")
                    }
                    *sample = Sample::from(&value);
                }
            },
            move |err| {},
        )
        .unwrap();
    stream
}
