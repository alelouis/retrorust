pub struct Lencounter{
    // runs at CPU frequency clock
    length: u16,
    value: u16, 
    enabled: bool
}

impl Lencounter {
    pub fn new(length: u16) -> Self {
        Lencounter {
            length: length,
            value: length, 
            enabled: false,
        }
    }
    pub fn tick(&mut self) {
        if self.enabled {
            self.value -= 1;    
            if self.value == 0 {
                self.enabled = false;
            }
        }

    }
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn is_enabled(&self) -> bool {
        return self.enabled;
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