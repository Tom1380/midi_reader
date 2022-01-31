use std::time::Duration;

use rodio::{source::SineWave, Source};

/// An infinite source that produces a sine.
///
/// Always has a rate of 48kHz and one channel.
#[derive(Clone, Debug)]
pub struct SquareWave {
    inner_sine_wave: SineWave,
}

impl SquareWave {
    /// The frequency of the sine.
    #[inline]
    pub fn new(freq: f32) -> SquareWave {
        SquareWave {
            inner_sine_wave: SineWave::new(freq),
        }
    }
}

impl Iterator for SquareWave {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        let sin = self.inner_sine_wave.next()?;

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
