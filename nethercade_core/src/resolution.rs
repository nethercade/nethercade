use bitcode::{Decode, Encode};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Encode, Decode)]
pub enum Resolution {
    Full, // 1920x1080
    #[default]
    High, // 960x540
    Retro, // 640x360
    Compact, // 480x270
}

impl Resolution {
    /// Returns the width and height of the resolution.
    pub fn dimensions(&self) -> (u32, u32) {
        match self {
            Resolution::Full => (1920, 1080),
            Resolution::High => (960, 540),
            Resolution::Retro => (640, 360),
            Resolution::Compact => (480, 270),
        }
    }
}
