/// Volume control
#[derive(Debug, Copy, Clone)]
pub struct Envelope {
    period: u16,      // Period of envelop if looping, otherwise its just its length.
    value: u16,       // Current counter value
    looping: bool,    // Loop the envelope
    enabled: bool,    // State flag
    increasing: bool, // Increasing or decreasing ramp
}

impl Envelope {
    /// Constructor
    pub fn new(period: u16, looping: bool, increasing: bool) -> Self {
        let value = match increasing {
            true => 0,
            false => period,
        };
        Envelope {
            period,
            value,
            looping,
            enabled: false,
            increasing,
        }
    }

    /// Cycle action
    ///
    /// If disabled, do nothing.
    /// If enabled, increase of decrease value counter.
    /// Checks bounds and reset if in looping mode, else disable and return.
    ///
    /// In case of no looping, disabling sets self.value to 0.
    pub fn tick(&mut self) {
        // Action only on enabled state.
        if self.enabled {
            // In increase mode.
            if self.increasing {
                // Counter on top of ramp.
                if self.value == self.period {
                    self.value = 0;

                    // If not looping, stop here.
                    if !self.looping {
                        self.disable();
                        return;
                    }
                }

                // Increase counter.
                self.value += 1;

            // In decrease mode
            } else {
                // Counter on bottom of ramp.
                if self.value == 0 {
                    self.value = self.period;

                    // If not looping, stop here.
                    if !self.looping {
                        self.disable();
                        return;
                    }
                }

                // Decrease counter.
                self.value -= 1;
            }
        }
    }

    /// Enable envelope, setting value to either 0
    /// or period for increasing and decreasing modes.
    pub fn enable(&mut self) {
        self.value = match self.increasing {
            true => 0,
            false => self.period,
        };
        self.enabled = true;
    }

    /// Disable envelope, returns 0 volume.
    pub fn disable(&mut self) {
        self.value = 0;
        self.enabled = false;
    }

    /// Returns current value (0 if disables or ramp value otherwise).
    pub fn get_value(&self) -> u16 {
        self.value
    }

    /// Returns envelop period
    pub fn get_period(&self) -> u16 {
        self.period
    }
}
