use bytemuck::{Pod, Zeroable};

use super::input::{InputState, MouseState};

#[derive(Pod, Zeroable, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct NetworkInputState {
    pub input_state: InputState,
    pub mouse_state: MouseState,
}
