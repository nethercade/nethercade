use serde::{Deserialize, Serialize};
use strum::EnumIter;

#[derive(Debug, Copy, Clone, EnumIter, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum ButtonCode {
    // DPad
    Up,
    Down,
    Left,
    Right,

    // Buttons
    ButtonA,
    ButtonB,
    ButtonC,
    ButtonD,
    Start,
    Select,
    LeftShoulder,
    RightShoulder,
    LeftStick,
    RightStick,

    // Emulated
    LeftTrigger,
    RightTrigger,
}

impl ToBitMask<u16> for ButtonCode {
    fn to_bit_mask(&self) -> u16 {
        match self {
            Self::Up => 0b100_0000,
            Self::Down => 0b1000_0000,
            Self::Left => 0b1_0000_0000,
            Self::Right => 0b10_0000_0000,
            Self::ButtonA => 0b1,
            Self::ButtonB => 0b10,
            Self::ButtonC => 0b100,
            Self::ButtonD => 0b1000,
            Self::Start => 0b1_0000,
            Self::Select => 0b10_0000,
            Self::LeftShoulder => 0b100_0000_0000,
            Self::RightShoulder => 0b1000_0000_0000,
            Self::LeftStick => 0b1_0000_0000_0000,
            Self::RightStick => 0b10_0000_0000_0000,
            Self::LeftTrigger => 0b100_0000_0000_0000,
            Self::RightTrigger => 0b1000_0000_0000_0000,
        }
    }
}

pub trait ToBitMask<T> {
    fn to_bit_mask(&self) -> T;
}
