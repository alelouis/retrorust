pub struct Timer {
    // runs at CPU frequency clock
    period: u16,
    value: u16
}

impl Timer {
    pub fn new(period: u16) -> Self {
        Timer {
            period: period,
            value: period,
        }
    }
    pub fn tick(&mut self) {
        if self.value == 0 {
            self.value = self.period;
        }
        self.value -= 1;

    }
    pub fn get_value(&self) -> u16 {
        self.value
    }
}