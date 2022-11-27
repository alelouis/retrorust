/// Lencounter
///
/// Use to silence channel after a given amount of time.
/// Redundant with envelop if not looping and length < envelop period

#[derive(Debug, Copy, Clone)]
pub struct Lencounter {
    // runs at CPU frequency clock
    length: u16,
    value: u16,
    enabled: bool,
}

impl Lencounter {
    /// Constructor
    pub fn new(length: u16) -> Self {
        Lencounter {
            length,
            value: length,
            enabled: false,
        }
    }

    /// Cycle action
    pub fn tick(&mut self) {
        // Only on enabled state.
        if self.enabled {
            // Decrease value.
            self.value -= 1;

            // On bottom of ramp, disable unit.
            if self.value == 0 {
                self.enabled = false;
            }
        }
    }

    /// Enables unit
    /// Set value to total length and set enable flag to true
    pub fn enable(&mut self) {
        self.value = self.length;
        self.enabled = true;
    }

    /// Returns enabled states
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

#[cfg(test)]
mod tests {
    // this brings everything from parent's scope into this scope
    use super::*;

    #[test]
    fn enable_check() {
        let mut length_counter = Lencounter::new(10u16);
        length_counter.enable();
        assert_eq!(length_counter.is_enabled(), true);
    }

    #[test]
    fn disable_check() {
        let mut length_counter = Lencounter::new(10u16);
        length_counter.enable();
        for _ in 0..10 {
            length_counter.tick()
        }
        assert_eq!(length_counter.is_enabled(), false)
    }
}
