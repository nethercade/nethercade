use serde::{Deserialize, Serialize};

use super::{FrameRate, Resolution};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rom {
    code: Vec<u8>,
    resolution: Resolution,
    frame_rate: FrameRate,
}
