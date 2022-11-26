use crate::lencounter::Lencounter;
use crate::sequencer::Sequencer;
use crate::timer::Timer;

pub struct Pulse {
    lencounter: Lencounter,
    timer: Timer,
    sequencer: Sequencer,
}

impl Pulse {
    pub fn new(frequency: f32, clock: f32) -> Self {
        let timer_period = (clock / (frequency*8.)) as u16;
        Pulse {
            lencounter: Lencounter::new(16u16),
            timer: Timer::new(timer_period),
            sequencer: Sequencer::new(),
        }
    }
    pub fn tick(&mut self) {
        self.timer.tick();
        if self.timer.get_value() == 0 {
            self.sequencer.tick()
        }
    }

    pub fn get_value(&self) -> i8 {
        return self.sequencer.get_sample();
    }
}
