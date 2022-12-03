// author: Rob Saunders <hello@robsaunders.io>

#[macro_use]
extern crate vst;

use vst::prelude::*;

use std::f64::consts::PI;

mod envelope;
mod lencounter;
mod pulse;
mod sequencer;
mod ticker;
mod timer;

use pulse::Pulse;

/// Convert the midi note's pitch into the equivalent frequency.
///
/// This function assumes A4 is 440hz.
fn midi_pitch_to_freq(pitch: u8) -> f64 {
    const A4_PITCH: i8 = 69;
    const A4_FREQ: f64 = 440.0;

    // Midi notes can be 0-127
    ((f64::from(pitch as i8 - A4_PITCH)) / 12.).exp2() * A4_FREQ
}

struct RetroSynth {
    sample_rate: f64,
    note: Option<u8>,
    pulse: Pulse,
}

impl RetroSynth {

    fn process_midi_event(&mut self, data: [u8; 3]) {
        match data[0] {
            0x90 => {
                self.pulse.set_frequency(midi_pitch_to_freq(data[1]) as f32);
                self.pulse.trigger();
            },
            _ => (),
        }
    }
}

pub const TAU: f64 = PI * 2.0;

impl Plugin for RetroSynth {
    fn new(_host: HostCallback) -> Self {
        let multiplier = 16_f32;
        let clock = multiplier * 44100_f32;
        let frequency = 440_f32;
        let pulse: Pulse = Pulse::new(frequency, clock);

        RetroSynth {
            sample_rate: 44100.0,
            note: None,
            pulse,
        }
    }

    fn get_info(&self) -> Info {
        Info {
            name: "RetroSynth".to_string(),
            vendor: "alelouis".to_string(),
            unique_id: 1111,
            category: Category::Synth,
            inputs: 2,
            outputs: 2,
            parameters: 0,
            initial_delay: 0,
            ..Info::default()
        }
    }

    #[allow(unused_variables)]
    #[allow(clippy::single_match)]
    fn process_events(&mut self, events: &Events) {
        for event in events.events() {
            match event {
                Event::Midi(ev) => self.process_midi_event(ev.data),
                // More events can be handled here.
                _ => (),
            }
        }
    }

    fn set_sample_rate(&mut self, rate: f32) {
        self.sample_rate = f64::from(rate);
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        let samples = buffer.samples();
        let (_, mut outputs) = buffer.split();
        let output_count = outputs.len();
        let mut output_sample;
        for sample_idx in 0..samples {
            for buf_idx in 0..output_count {
                for _ in 0..16 {
                    self.pulse.tick();
                }
                output_sample = 0.5 * self.pulse.get_value() as f32;
                let buff = outputs.get_mut(buf_idx);
                buff[sample_idx] = output_sample;
            }
        }
    }

    fn can_do(&self, can_do: CanDo) -> Supported {
        match can_do {
            CanDo::ReceiveMidiEvent => Supported::Yes,
            _ => Supported::Maybe,
        }
    }
}

plugin_main!(RetroSynth);
