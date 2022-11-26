#![allow(dead_code)]
#![allow(unused_variables)]

mod timer;
mod sequencer;
mod lencounter;
mod pulse;

fn main() {
    let f_clk = 44100_u32;


    let mut timer = timer::Timer::new(10u16);
    let mut sequencer = sequencer::Sequencer::new();
    loop {
        timer.tick();
        if timer.get_value() == 0 {
            println!("Ticking at {}", sequencer.get_sample());
            sequencer.tick();
        }
    }
}
