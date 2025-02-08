use std::net::SocketAddr;

use bytemuck::{Pod, Zeroable};
use ggrs::PlayerType;
use serde::{Deserialize, Serialize};

use super::input::{Buttons, InputState, MouseState};

#[derive(Pod, Zeroable, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[repr(C)]
pub struct NetworkInputState {
    pub input_state: InputState,
    pub mouse_state: MouseState,
}

#[derive(Clone)]
pub struct WasmConsoleState {
    pub previous_buttons: Box<[Buttons]>,
    pub memory: Vec<u8>,
}

#[derive(Clone)]
pub struct SessionDescriptor {
    pub num_players: usize,
    pub player_types: Box<[PlayerType<SocketAddr>]>,
    pub port: u16,
}
