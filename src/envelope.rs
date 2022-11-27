pub struct Envelope {
    period: u16,
    value: u16,
    looping: bool, // Loop the envelope
    enabled: bool,
    increasing: bool, // Increasing or decreasing ramp
}

impl Envelope {
    pub fn new(period: u16, looping: bool, increasing: bool) -> Self {
        let value = match increasing {
            true => 0,
            false => period
        };
        Envelope {
            period,
            value: value,
            looping,
            enabled: false,
            increasing
        }
    }
    pub fn tick(&mut self) {
        if self.enabled {
            if self.increasing {
                if self.value == self.period {
                    self.value = 0;
                    if !self.looping {
                        self.disable();
                        return
                    }
                }
                self.value += 1;
            } else {
                if self.value == 0 {
                    self.value = self.period;
                    if !self.looping {
                        self.disable();
                        return
                    }
                }
                self.value -= 1;
            }
        }
    }
    pub fn enable(&mut self) {
        self.value = match self.increasing {
            true => 0,
            false => self.period
        };
        self.enabled = true;
    }
    pub fn disable(&mut self) {
        self.value = 0;
        self.enabled = false;
    }
    pub fn get_value(&self) -> u16 {
        self.value
    }
    pub fn get_period(&self) -> u16 {
        return self.period;
    }
}