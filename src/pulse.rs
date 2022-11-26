use crate::lencounter::Lencounter;
use crate::timer::Timer;
use crate::sequencer::Sequencer;


pub struct Pulse{
    lencounter: Lencounter,
    timer: Timer, 
    sequencer: Sequencer
}

impl Pulse {
    pub fn new() -> Self {
        Pulse {
            lencounter: Lencounter::new(32u16),
            timer: Timer::new(8u16), 
            sequencer: Sequencer::new(),
        }
    }
    pub fn tick(&mut self) {
        self.lencounter.tick();
        self.timer.tick();
        if self.timer.get_value() == 0 {
            self.sequencer.tick()
        }
    }

    pub fn get_value(&self) -> i8 {
        return self.sequencer.get_sample()
    }

}

#[cfg(test)]
mod tests {
    // this brings everything from parent's scope into this scope
    use super::*;

    #[test]
    fn run() {
        let mut pulse = Pulse::new();
        for _ in 0..64 {
            println!("{:?}", pulse.get_value());
            pulse.tick();
        }
    }
}