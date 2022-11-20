#![allow(dead_code)]

mod timer;
mod sequencer;

fn main() {
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
