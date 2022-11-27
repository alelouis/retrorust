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
use std::sync::mpsc::channel;

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
                Ok(s) => {}
                Err(e) => break,
            }
        }

        let mut start_index = 0;
        let mut buffer_phase: [f32; 4 * WIDTH] = [0.; 4 * WIDTH];

        for i in 0..4 * WIDTH {
            buffer_phase[i] = rx.recv().unwrap();
            if i > WIDTH && i < 3 * WIDTH {
                if buffer_phase[i + 1] > buffer_phase[i] {
                    start_index = i;
                }
            }
        }

        for column in 0..WIDTH {
            let index = (column as f32 + start_index as f32) - WIDTH as f32;
            waveform[column] = buffer_phase[column + index as usize];
        }

        for c in 0..WIDTH {
            let row = (((waveform[c] + 1.) / 2.) * (HEIGHT - 5) as f32) as usize;
            if c > 1 {
                let prev_row = (((waveform[c - 1] + 1.) / 2.) * (HEIGHT - 5) as f32) as usize;
                let row_min = std::cmp::min(row, prev_row);
                let row_max = std::cmp::max(row, prev_row);
                for row_i in row_min..=row_max {
                    buffer_mat[c][row_i] = from_u8_rgb(255, 255, 255);
                    buffer_mat[c][row_i - 1] = from_u8_rgb(255, 255, 255);
                }
            } else {
                buffer_mat[c][row] = from_u8_rgb(255, 255, 255);
                buffer_mat[c][row - 1] = from_u8_rgb(255, 255, 255);
            }
        }

        for (idx, value) in buffer.iter_mut().enumerate() {
            let row = idx / WIDTH;
            let column = idx % WIDTH;
            *value = buffer_mat[column][row];
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
