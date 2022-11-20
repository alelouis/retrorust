pub struct Sequencer {
    sequence: [i8; 8],
    position: usize,
}

impl Sequencer {
    pub fn new() -> Self {
        Sequencer {
            sequence: [1, 1, 1, 1, 1, 1, -1, -1],
            position: 0,
        }
    }
    pub fn tick(&mut self) {
        self.position += 1;
        if self.position == 8 {
            self.position = 0;
        }
    }
    pub fn get_position(&self) -> usize {
        self.position
    }

    pub fn get_sample(&self) -> i8 {
        self.sequence[self.position]
    }
}