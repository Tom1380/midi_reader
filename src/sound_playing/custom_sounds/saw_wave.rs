use std::time::Duration;

use rodio::Source;

/// Always has a rate of 48kHz and one channel.
#[derive(Clone, Debug)]
pub struct SawWave {
    freq: f32,
    num_sample: usize,
}

impl SawWave {
    #[inline]
    pub fn new(freq: f32) -> SawWave {
        SawWave {
            freq,
            num_sample: 0,
        }
    }
}

impl Iterator for SawWave {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        self.num_sample = self.num_sample.wrapping_add(1);

        let t = self.num_sample as f32 / self.sample_rate() as f32;
        let result = 2.0 * (t * self.freq - (1.0 / 2.0 + t * self.freq).floor());

        Some(result)
    }
}

impl Source for SawWave {
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
