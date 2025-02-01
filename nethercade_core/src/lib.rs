mod resolution;
pub use resolution::*;

mod rom;
pub use rom::*;

mod frame_rate;
pub use frame_rate::*;

pub const AUDIO_SAMPLE_RATE: u32 = 32_400; //32.4khz
