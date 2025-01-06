use bitcode::{Decode, Encode};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Encode, Decode)]
pub enum FrameRate {
    UltraFast,
    #[default]
    Fast,
    Retro,
    Cinematic,
}

impl FrameRate {
    pub const fn as_str(&self) -> &str {
        match self {
            FrameRate::UltraFast => "Ultra Fast",
            FrameRate::Fast => "Fast",
            FrameRate::Retro => "Retro",
            FrameRate::Cinematic => "Cinematic",
        }
    }

    pub const fn frames_per_second(self) -> usize {
        match self {
            FrameRate::UltraFast => 120,
            FrameRate::Fast => 60,
            FrameRate::Retro => 30,
            FrameRate::Cinematic => 24,
        }
    }

    pub const fn default_input_delay(self) -> usize {
        match self {
            FrameRate::UltraFast => 3,
            FrameRate::Fast => 2,
            FrameRate::Retro => 1,
            FrameRate::Cinematic => 0,
        }
    }

    pub fn frame_time(self) -> f32 {
        (self.frames_per_second() as f32).recip()
    }
}
