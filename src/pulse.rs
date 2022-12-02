use crate::envelope::Envelope;
use crate::lencounter::Lencounter;
use crate::sequencer::Sequencer;
use crate::ticker::Ticker;
use crate::timer::Timer;

#[derive(Debug, Copy, Clone)]
pub struct Pulse {
    lencounter: Lencounter,
    timer: Timer,
    sequencer: Sequencer,
    envelope: Envelope,
    clock: f32,
}

impl Pulse {
    // TODO: Increase the clock frequency
    pub fn new(frequency: f32, clock: f32) -> Self {
        let timer_period = (clock / (frequency * 8.)) as u16;
        Pulse {
            clock,
            lencounter: Lencounter::new(clock as u32),
            timer: Timer::new(timer_period),
            sequencer: Sequencer::new(3),
            envelope: Envelope::new(clock as u32, true, false),
        }
    }

    /// Sets wave frequency
    pub fn set_frequency(&mut self, frequency: f32) {
        let new_period = (self.clock / (frequency * 8.)) as u16;
        self.update_timer_period(new_period);
    }

    /// Updates internal timer period
    fn update_timer_period(&mut self, period: u16) {
        self.timer.set_period(period);
    }


    /// Cycle action
    ///
    /// Ticks all internal units
    pub fn tick(&mut self) {
        self.timer.tick();
        self.lencounter.tick();
        self.envelope.tick();
        if self.timer.get_value() == 0 {
            self.sequencer.tick()
        }
    }

    /// Compute output value of Pulse unit
    pub fn get_value(&self) -> f32 {
        match self.lencounter.is_enabled() {
            true => {
                let volume = self.envelope.get_value() as f32 / self.envelope.get_period() as f32;
                let sample = self.sequencer.get_sample() as f32;
                volume * sample
            }
            false => 0_f32,
        }
    }

    /// Trigger length counter and envelope units
    pub fn trigger(&mut self) {
        self.lencounter.enable();
        self.envelope.enable();
    }
}
