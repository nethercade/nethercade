use std::path::PathBuf;

use nethercade_core::{FrameRate, Resolution};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub resolution: Option<Resolution>,
    pub frame_rate: Option<FrameRate>,
    pub wasm_path: PathBuf,
    pub output_file: Option<PathBuf>,
}
