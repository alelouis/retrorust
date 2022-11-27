#![allow(dead_code)]
#![allow(unused_variables)]

extern crate minifb;

mod audio;
mod csv_rw;
mod envelope;
mod lencounter;
mod pulse;
mod sequencer;
mod timer;

use crate::pulse::Pulse;
use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 500;
const HEIGHT: usize = 160;

fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let clock = 44100_f32;
    let frequency = 440_f32;

    let mut pulse: Pulse = Pulse::new(frequency, clock);
    pulse.trigger();

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut waveform: Vec<f32> = vec![0.; WIDTH];
    let mut column: usize = 0;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        pulse.tick();

        let mut buffer_mat: Vec<Vec<u32>> = vec![vec![0; HEIGHT]; WIDTH];
        column += 1;
        // Wave position
        if column == WIDTH {
            column = 0;
        }

        // Building buffer
        waveform[column] = (pulse.get_value() as f32) / 16.;
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
