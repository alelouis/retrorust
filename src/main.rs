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
use minifb::{Key, Window, WindowOptions};
use pulse::Pulse;
use std::sync::mpsc::{channel};

const WIDTH: usize = 512;
const HEIGHT: usize = 100;

fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

fn main() {
    // Graphics
    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Synthesizer
    let clock = 44100_f32;
    let frequency = 440_f32;
    let pulse: Pulse = Pulse::new(frequency, clock);

    // Audio
    let (tx, rx) = channel();
    let stream = audio::stream(pulse, tx);
    
    stream.play().unwrap();

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    window.limit_update_rate(Some(std::time::Duration::from_micros(100)));

    let mut waveform: Vec<f32> = vec![0.; WIDTH];
    while window.is_open() && !window.is_key_down(Key::Escape) {

        let mut buffer_mat: Vec<Vec<u32>> = vec![vec![0; HEIGHT]; WIDTH];

        loop {
            match rx.try_recv() {
                Ok(s) => {},
                Err(e) => break
            }
        }

        for column in 0..WIDTH {
            let value = rx.recv().unwrap();
            waveform[column] = value;
        }

        for c in 0..WIDTH {
            let row = (((waveform[c] + 1.) / 2.) * (HEIGHT - 1) as f32) as usize;
            buffer_mat[c][row] = from_u8_rgb(255, 255, 255);
        }

        for (idx, value) in buffer.iter_mut().enumerate() {
            let row = idx / WIDTH;
            let column = idx % WIDTH;
            *value = buffer_mat[column][row];
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
