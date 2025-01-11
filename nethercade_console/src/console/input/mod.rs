mod gamepad_bindings;
mod input_code;
mod input_state;
mod key_bindings;
mod key_types;
mod local_input_manager;
mod mouse_state;
mod player_input_entry;

pub use input_code::*;
pub use input_state::*;
pub use local_input_manager::{LocalInputManager, MouseEventCollector};
pub use mouse_state::*;
pub use player_input_entry::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum InputMode {
    Emulated(LocalKeyboardId),
    Gamepad(gilrs::GamepadId),
}

use serde::{Deserialize, Serialize};
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
pub struct LocalKeyboardId(pub usize);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
pub struct LocalPlayerId(pub usize);
