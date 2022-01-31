mod saw_wave;
mod square_wave;

pub use rodio::source::SineWave;
pub use saw_wave::SawWave;
pub use square_wave::SquareWave;

use rodio::{Sample, Source};
pub trait CustomSound: Source
where
    <Self as Iterator>::Item: Sample,
{
    fn new(freq: f32) -> Self;
}

// I know it seems counter intuitive.
// We need to implement CustomSound for SineWave, but the functionality is already there.
impl CustomSound for SineWave {
    fn new(freq: f32) -> SineWave {
        SineWave::new(freq)
    }
}
