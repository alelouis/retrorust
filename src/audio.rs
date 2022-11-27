use cpal::traits::{DeviceTrait, HostTrait};
use cpal::{BufferSize, Device, Sample, SampleRate, Stream, StreamConfig};
use device_query::{DeviceQuery, DeviceState, Keycode};
use std::sync::mpsc::Sender;
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

fn react_on_key(keys: &Vec<Keycode>, pulse: &mut Pulse) {
    if keys.contains(&Keycode::Up) {
        pulse.set_frequency(220_f32);
        pulse.trigger();
    }
    if keys.contains(&Keycode::Down) {
        pulse.set_frequency(330_f32);
        pulse.trigger();
    }
}

/// Build stream object
pub fn stream(mut pulse: Pulse, tx: Sender<f32>) -> Stream {
    let device = setup_device();
    let config = setup_stream_config();
    let device_state = DeviceState::new();
    let mut prev_keys = vec![];

    let stream = device
        .build_output_stream(
            &config,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {

                // Keys triggering
                let keys = device_state.get_keys();
                if keys != prev_keys {
                    react_on_key(&keys, &mut pulse)
                }
                prev_keys = keys;

                // Buffer filling
                for sample in data.iter_mut() {
                    pulse.tick();
                    let norm_value = (pulse.get_value() as f32) / 16.;
                    tx.send(norm_value).unwrap();
                    let value: f32 = 0.05 * norm_value;
                    *sample = Sample::from(&value);
                }
            },
            move |err| {},
        )
        .unwrap();
    stream
}
