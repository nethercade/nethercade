use bitcode::{Decode, Encode};
use serde::{Deserialize, Serialize};

use super::{FrameRate, Resolution};

#[derive(Debug, Clone, Deserialize, Serialize, Encode, Decode)]
pub struct Rom {
    pub code: Box<[u8]>,
    pub resolution: Resolution,
    pub frame_rate: FrameRate,
}

impl Rom {
    pub fn from_code(code: &[u8]) -> Self {
        Self {
            code: code.to_vec().into_boxed_slice(),
            resolution: Resolution::default(),
            frame_rate: FrameRate::default(),
        }
    }
}
