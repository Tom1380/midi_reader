use super::CustomSound;
use rodio::Source;
use std::time::Duration;

/// Always has a rate of 48kHz and one channel.
#[derive(Clone, Debug)]
pub struct SquareWave {
    freq: f32,
    num_sample: usize,
}

impl CustomSound for SquareWave {
    fn new(freq: f32) -> SquareWave {
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

        let t = self.num_sample as f32 / self.sample_rate() as f32;
        let ft = self.freq * t;
        let result = 2.0 * (2.0 * ft.floor() - (2.0 * ft).floor()) + 1.0;

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
