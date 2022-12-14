use crate::pulse::Pulse;
use cpal::traits::{DeviceTrait, HostTrait};
use cpal::{BufferSize, Device, Sample, SampleRate, Stream, StreamConfig};
use device_query::{DeviceState, Keycode};
use std::sync::mpsc::{Receiver, Sender};

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
    if keys.contains(&Keycode::F6) {
        pulse.set_frequency(440_f32);
        pulse.trigger();
    }
    if keys.contains(&Keycode::F7) {
        pulse.set_frequency(466.16_f32);
        pulse.trigger();
    }
    if keys.contains(&Keycode::F8) {
        pulse.set_frequency(493.88_f32);
        pulse.trigger();
    }
    if keys.contains(&Keycode::F9) {
        pulse.set_frequency(523.25_f32);
        pulse.trigger();
    }
    if keys.contains(&Keycode::F10) {
        pulse.set_frequency(554.37_f32);
        pulse.trigger();
    }
}

/// Build stream object
pub fn stream(mut pulse: Pulse, tx: Sender<f32>, rx: Receiver<f32>) -> Stream {
    let device = setup_device();
    let config = setup_stream_config();
    let device_state = DeviceState::new();
    //let mut prev_keys = vec![];

    let stream = device
        .build_output_stream(
            &config,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                // Keys triggering
                /*
                let keys = device_state.get_keys();
                if keys != prev_keys {
                    react_on_key(&keys, &mut pulse)
                }
                prev_keys = keys;
                */
                rx.try_recv()
                    .and_then(|frequency| {
                        Ok({
                            pulse.set_frequency(frequency);
                            pulse.trigger();
                        })
                    })
                    .ok();

                // Buffer filling
                for sample in data.iter_mut() {
                    for _ in 0..16 {
                        pulse.tick();
                    }
                    let norm_value = pulse.get_value();
                    tx.send(norm_value).unwrap();
                    let value: f32 = 0.5 * norm_value;
                    *sample = Sample::from(&value);
                }
            },
            move |err| {},
        )
        .unwrap();
    stream
}
