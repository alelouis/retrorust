/// Timer unit
#[derive(Debug, Copy, Clone)]
pub struct Timer {
    // runs at CPU frequency clock
    period: u16,
    value: u16,
}

impl Timer {
    /// Constructor
    pub fn new(period: u16) -> Self {
        Timer {
            period,
            value: period,
        }
    }

    /// Cycle action
    ///
    /// Timer is a decreasing sawtooth that resets in infinite loop.
    pub fn tick(&mut self) {
        // Goes from self.period -1 to 0, so self.period ticks.
        if self.value == 0 {
            self.value = self.period;
        }
        self.value -= 1;
    }

    /// Sets new periods and resets value
    pub fn set_period(&mut self, period: u16) {
        self.period = period;
        self.value = period;
    }

    /// Returns current counter value
    pub fn get_value(&self) -> u16 {
        self.value
    }
}

#[cfg(test)]
mod tests {
    // this brings everything from parent's scope into this scope
    use super::*;

    #[test]
    fn period_check() {
        let mut timer = Timer::new(10u16);
        for _ in 0..10 {
            timer.tick()
        }
        assert_eq!(timer.get_value(), 0)
    }
}
