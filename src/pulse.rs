use crate::envelope::Envelope;
use crate::lencounter::Lencounter;
use crate::sequencer::Sequencer;
use crate::timer::Timer;

pub struct Pulse {
    lencounter: Lencounter,
    timer: Timer,
    sequencer: Sequencer,
    envelope: Envelope,
}

impl Pulse {
    pub fn new(frequency: f32, clock: f32) -> Self {
        let timer_period = (clock / (frequency * 8.)) as u16;
        Pulse {
            lencounter: Lencounter::new(44100u16),
            timer: Timer::new(timer_period),
            sequencer: Sequencer::new(4),
            envelope: Envelope::new(44100u16 / 4, true, false),
        }
    }
    pub fn tick(&mut self) {
        self.timer.tick();
        self.lencounter.tick();
        self.envelope.tick();
        if self.timer.get_value() == 0 {
            self.sequencer.tick()
        }
    }

    pub fn get_value(&self) -> i8 {
        match self.lencounter.is_enabled() {
            true => {
                let volume = self.envelope.get_value() as f32 / self.envelope.get_period() as f32;
                let sample = self.sequencer.get_sample() as f32;
                (16. * (volume * sample)) as i8
            }
            false => 1,
        }
    }

    pub fn trigger(&mut self) {
        self.lencounter.enable();
        self.envelope.enable();
    }
}
