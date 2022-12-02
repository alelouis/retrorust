#![allow(dead_code)]
#![allow(unused_variables)]

mod audio;
mod envelope;
mod lencounter;
mod pulse;
mod sequencer;
mod timer;

use cpal::traits::StreamTrait;
use minifb::{Key, Window, WindowOptions};
use pulse::Pulse;
use std::sync::mpsc::channel;

const WIDTH: usize = 256;
const HEIGHT: usize = 100;

fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

fn main() {
    // Graphics
    let mut window = Window::new("Retrorust", WIDTH, HEIGHT, WindowOptions::default())
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

    // Synthesizer
    let multiplier = 16_f32;
    let clock = multiplier * 44100_f32;
    let frequency = 440_f32;
    let pulse: Pulse = Pulse::new(frequency, clock);

    // Audio
    let (tx, rx) = channel();
    let stream = audio::stream(pulse, tx);

    stream.play().unwrap();

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let mut buffer_mat: Vec<Vec<u32>> = vec![vec![0; HEIGHT]; WIDTH];

        loop {
            match rx.try_recv() {
                Ok(s) => {}
                Err(e) => break,
            }
        }

        let mut start_index = 0;
        let mut buffer_phase: [f32; 2 * WIDTH] = [0.; 2 * WIDTH];

        for i in 0..2 * WIDTH {
            buffer_phase[i] = rx.recv().unwrap();
            if i > (0.5 * WIDTH as f32) as usize && i < (1.5 * WIDTH as f32) as usize {
                if buffer_phase[i] > buffer_phase[i - 1] {
                    start_index = i;
                }
            }
        }

        for c in 0..WIDTH {
            let index = (c as f32 + start_index as f32) - (WIDTH / 2) as f32;
            let mut row = HEIGHT
                - (((buffer_phase[index as usize] + 1.) / 2.) * (HEIGHT - 1) as f32) as usize;
            row = std::cmp::min(row, HEIGHT - 1);
            if c > 1 {
                let mut prev_row = HEIGHT
                    - (((buffer_phase[(index - 1.) as usize] + 1.) / 2.) * (HEIGHT - 1) as f32)
                        as usize;
                prev_row = std::cmp::min(prev_row, HEIGHT - 1);
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
