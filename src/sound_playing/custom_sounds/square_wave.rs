use std::f32::consts::PI;
use std::time::Duration;

use rodio::Source;

/// An infinite source that produces a sine.
///
/// Always has a rate of 48kHz and one channel.
#[derive(Clone, Debug)]
pub struct SquareWave {
    freq: f32,
    num_sample: usize,
}

impl SquareWave {
    /// The frequency of the sine.
    #[inline]
    pub fn new(freq: f32) -> SquareWave {
        SquareWave {
            freq,
            num_sample: 0,
        }
    }
}

impl Iterator for SquareWave {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        self.num_sample = self.num_sample.wrapping_add(1);

        let value = 2.0 * PI * self.freq * self.num_sample as f32 / self.sample_rate() as f32;
        let sin = value.sin();

        // Simply compute the sine wave and analyse its sign.
        let result = if sin == 0.0 {
            0.0
        } else if sin > 0.0 {
            1.0
        } else {
            -1.0
        };
        Some(result)
    }
}

impl Source for SquareWave {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        48000
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}
