use std::path::PathBuf;

use eframe::egui::Key;
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};

use super::{
    key_types::{AnalogStick, KeyType, TriggerSide},
    ButtonCode,
};

const INPUT_FILE_NAME: &str = "keyboardInput.json";

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub(crate) struct KeyBindings {
    pub buttons: Vec<HashMap<Key, KeyType>>,
}

impl KeyBindings {
    pub fn load() -> Self {
        let path = PathBuf::from(INPUT_FILE_NAME);
        if path.exists() {
            match std::fs::read(INPUT_FILE_NAME) {
                Ok(file) => match sonic_rs::from_slice::<Self>(&file) {
                    Ok(key_bindings) => {
                        println!("Successfully loaded key bindings from: {INPUT_FILE_NAME}",);
                        return key_bindings;
                    }
                    Err(e) => {
                        println!("{INPUT_FILE_NAME} found, but unable to parse: {e}");
                    }
                },
                Err(e) => println!("{INPUT_FILE_NAME} found, but unable to read: {e}"),
            };

            println!("Using default config.");
            Self::default()
        } else {
            println!("{INPUT_FILE_NAME} not found. Generating default input file.");
            let bindings = Self::default();

            let json = sonic_rs::to_string_pretty(&bindings).unwrap();

            match std::fs::write(path, json) {
                Ok(()) => println!("Successfully generated default {INPUT_FILE_NAME}"),
                Err(e) => println!("Error writing {INPUT_FILE_NAME}: {e}"),
            };

            bindings
        }
    }
}

impl Default for KeyBindings {
    fn default() -> Self {
        let buttons = vec![[
            //Sticks
            (Key::X, KeyType::Button(ButtonCode::LeftStick)),
            (Key::B, KeyType::Button(ButtonCode::RightStick)),
            //Shoulders
            (Key::E, KeyType::Button(ButtonCode::LeftShoulder)),
            (Key::Q, KeyType::Trigger(TriggerSide::LeftTrigger)),
            (Key::R, KeyType::Button(ButtonCode::RightShoulder)),
            (Key::Y, KeyType::Trigger(TriggerSide::RightTrigger)),
            //DPad:
            (Key::ArrowUp, KeyType::Button(ButtonCode::Up)),
            (Key::ArrowDown, KeyType::Button(ButtonCode::Down)),
            (Key::ArrowLeft, KeyType::Button(ButtonCode::Left)),
            (Key::ArrowRight, KeyType::Button(ButtonCode::Right)),
            //Buttons:
            (Key::U, KeyType::Button(ButtonCode::ButtonA)),
            (Key::I, KeyType::Button(ButtonCode::ButtonB)),
            (Key::J, KeyType::Button(ButtonCode::ButtonC)),
            (Key::K, KeyType::Button(ButtonCode::ButtonD)),
            (Key::Num5, KeyType::Button(ButtonCode::Start)),
            (Key::Num6, KeyType::Button(ButtonCode::Select)),
            //Left Stick Axis
            (Key::W, KeyType::AnalogStick(AnalogStick::LeftYPositive)),
            (Key::S, KeyType::AnalogStick(AnalogStick::LeftYNegative)),
            (Key::A, KeyType::AnalogStick(AnalogStick::LeftXNegative)),
            (Key::D, KeyType::AnalogStick(AnalogStick::LeftXPositive)),
            //Right Stick Axis,
            (Key::T, KeyType::AnalogStick(AnalogStick::RightYPositive)),
            (Key::G, KeyType::AnalogStick(AnalogStick::RightYNegative)),
            (Key::F, KeyType::AnalogStick(AnalogStick::RightXNegative)),
            (Key::H, KeyType::AnalogStick(AnalogStick::RightXPositive)),
        ]
        .into_iter()
        .collect::<HashMap<Key, KeyType>>()];

        Self { buttons }
    }
}
