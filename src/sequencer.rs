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
    samples: [f32; 16],
}

impl Sequence {
    /// Returns samples from the sequence
    pub fn get_samples(&self) -> [f32; 16] {
        self.samples
    }

    // Return varying duty cycle sequences for duty between 1 and 7.
    pub fn get_square_sequence_from_duty(duty: i8) -> Option<Sequence> {
        let mut samples = [1.0; 16];
        if (1..=15).contains(&duty) {
            for i in 0..duty {
                samples[i as usize] = -1.0;
            }
            Some(Sequence { samples })
        } else {
            None
        }
    }

    // Return triangle sequence.
    pub fn get_triangle_sequence() -> Sequence {
        let mut triangle = [0., 1., 2., 3., 4., 3., 2., 1., 0., -1., -2., -3., -4., -3., -2., -1.];
        for v in triangle.iter_mut() {
            *v = *v / 4.
        }
        Sequence { samples : triangle}
    }
}

impl Sequencer {
    pub fn new(sequence: Sequence) -> Self {
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
    pub fn get_sample(&self) -> f32 {
        self.sequence.get_samples()[self.position]
    }
}

impl Ticker for Sequencer {
    fn tick(&mut self) {
        self.position += 1;
        if self.position == 16 {
            self.position = 0;
        }
    }
}
