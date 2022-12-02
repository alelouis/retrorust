use crate::ticker::Ticker;
/// Sequencer unit
#[derive(Debug, Copy, Clone)]
pub struct Sequencer {
    sequence: Sequence,
    position: usize,
}

/// Sequence type, to be used by Sequencer
#[derive(Debug, Copy, Clone)]
pub struct Sequence {
    samples: [i8; 8],
}

impl Sequence {
    /// Returns samples from the sequence
    pub fn get_samples(&self) -> [i8; 8] {
        self.samples
    }

    // Return varying duty cycle sequences for duty between 1 and 7.
    pub fn get_sequence_from_duty(duty: i8) -> Option<Sequence> {
        let mut samples = [1, 1, 1, 1, 1, 1, 1, 1];
        if (1..=7).contains(&duty) {
            for i in 0..duty {
                samples[i as usize] = -1;
            }
            Some(Sequence { samples })
        } else {
            None
        }
    }
}

impl Sequencer {
    pub fn new(duty: i8) -> Self {
        let sequence = match Sequence::get_sequence_from_duty(duty) {
            Some(s) => s,
            None => Sequence {
                samples: [-1, -1, -1, -1, 1, 1, 1, 1],
            },
        };
        Sequencer {
            sequence,
            position: 0,
        }
    }

    /// Get sample for given sequence position
    pub fn get_position(&self) -> usize {
        self.position
    }

    /// Get sample for given sequence position
    pub fn get_sample(&self) -> i8 {
        self.sequence.get_samples()[self.position]
    }
}

impl Ticker for Sequencer {
    fn tick(&mut self) {
        self.position += 1;
        if self.position == 8 {
            self.position = 0;
        }
    }
}
