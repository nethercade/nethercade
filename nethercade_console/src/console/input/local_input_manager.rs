use std::collections::HashSet;

use egui::{Key, Vec2};
use gilrs::{Axis, Button, Gamepad, GamepadId, Gilrs};

use crate::console::network::NetworkInputState;

use super::{
    AnalogStick as AS, ButtonCode, InputMode, InputState, LocalKeyboardId, LocalPlayerId,
    MouseState,
    gamepad_bindings::GamepadBindings,
    key_bindings::KeyBindings,
    key_types::{AnalogStick, KeyType, TriggerSide},
};

#[derive(Default)]
pub struct MouseEventCollector {
    pub wheel_up: bool,
    pub wheel_down: bool,
    pub wheel_left: bool,
    pub wheel_right: bool,
    pub delta_x: i16,
    pub delta_y: i16,
    pub button_left: bool,
    pub button_right: bool,
    pub button_middle: bool,
}

#[derive(Debug)]
pub struct LocalInputManager {
    pub(crate) keyboard_bindings: KeyBindings,
    gamepad_binds: GamepadBindings,
    pub(crate) player_bindings: Vec<InputMode>,
}

impl LocalInputManager {
    pub fn new() -> Self {
        Self {
            keyboard_bindings: KeyBindings::load(),
            gamepad_binds: GamepadBindings::default(),
            player_bindings: vec![InputMode::Emulated(LocalKeyboardId(0))],
        }
    }

    pub fn generate_input_state(
        &self,
        local_player: LocalPlayerId,
        mouse_events: &MouseEventCollector,
        mouse_pos: Option<Vec2>,
        held_keys: &HashSet<Key>,
        gilrs: &Gilrs,
    ) -> NetworkInputState {
        let input_state = match self.player_bindings.get(local_player.0) {
            Some(InputMode::Emulated(keyboard_id)) => {
                self.new_emulated_state(*keyboard_id, held_keys)
            }
            Some(InputMode::Gamepad(gamepad_id)) => self.new_gamepad_state(*gamepad_id, gilrs),
            None => InputState::default(),
        };

        let mouse_state = generate_mouse_state(mouse_pos, mouse_events);

        NetworkInputState {
            input_state,
            mouse_state,
        }
    }

    fn new_emulated_state(
        &self,
        keyboard_id: LocalKeyboardId,
        held_keys: &HashSet<Key>,
    ) -> InputState {
        generate_emulated_state(keyboard_id, &self.keyboard_bindings, held_keys)
    }

    fn new_gamepad_state(&self, id: GamepadId, gilrs: &Gilrs) -> InputState {
        if let Some(gamepad) = gilrs.connected_gamepad(id) {
            generate_gamepad_state(&self.gamepad_binds, &gamepad)
        } else {
            InputState::default()
        }
    }
}

fn generate_gamepad_state(binds: &GamepadBindings, gamepad: &Gamepad) -> InputState {
    let mut output = InputState::default();

    binds.buttons.iter().for_each(|(button, input)| {
        if gamepad.is_pressed(*button) {
            output.buttons.enable_button(*input);
        }
    });

    if let Some(axis) = gamepad.axis_data(Axis::LeftStickX) {
        output.left_stick.set_x_axis(axis.value())
    }
    if let Some(axis) = gamepad.axis_data(Axis::LeftStickY) {
        output.left_stick.set_y_axis(axis.value())
    }

    if let Some(axis) = gamepad.axis_data(Axis::RightStickX) {
        output.right_stick.set_x_axis(axis.value())
    }
    if let Some(axis) = gamepad.axis_data(Axis::RightStickY) {
        output.right_stick.set_y_axis(axis.value())
    }

    if let Some(trigger) = gamepad.button_data(Button::LeftTrigger2) {
        output.left_trigger.set_value(trigger.value())
    }
    if let Some(trigger) = gamepad.button_data(Button::RightTrigger2) {
        output.right_trigger.set_value(trigger.value())
    }

    output
}

fn generate_emulated_state(
    player_id: LocalKeyboardId,
    binds: &KeyBindings,
    held_keys: &HashSet<Key>,
) -> InputState {
    let mut output = InputState::default();

    if let Some(buttons) = binds.buttons.get(player_id.0) {
        buttons.iter().for_each(|(code, input)| {
            if held_keys.contains(code) {
                match input {
                    KeyType::Button(code) => output.buttons.enable_button(*code),
                    KeyType::AnalogStick(analog_stick) => {
                        adjust_input_state(analog_stick, &mut output)
                    }
                    KeyType::Trigger(side) => match side {
                        TriggerSide::LeftTrigger => {
                            output.buttons.enable_button(ButtonCode::LeftTrigger);
                            output.left_trigger.set_value(1.0);
                        }
                        TriggerSide::RightTrigger => {
                            output.buttons.enable_button(ButtonCode::RightTrigger);
                            output.right_trigger.set_value(1.0)
                        }
                    },
                }
            }
        });
    }

    output
}

fn generate_mouse_state(mouse_pos: Option<Vec2>, mouse_events: &MouseEventCollector) -> MouseState {
    let mut out = MouseState::default();

    match mouse_pos {
        Some(pos) => {
            out.set_x_pos(pos.x as u32);
            out.set_y_pos(pos.y as u32);
        }
        _ => {
            out.set_x_pos(u32::MAX);
            out.set_y_pos(u32::MAX);
        }
    }

    out.set_left_button(mouse_events.button_left);
    out.set_right_button(mouse_events.button_right);
    out.set_middle_button(mouse_events.button_middle);

    out.set_x_delta(mouse_events.delta_x as i32);
    out.set_y_delta(mouse_events.delta_y as i32);

    out.set_wheel_up(mouse_events.wheel_up);
    out.set_wheel_down(mouse_events.wheel_down);
    out.set_wheel_left(mouse_events.wheel_left);
    out.set_wheel_right(mouse_events.wheel_right);

    out
}

fn adjust_input_state(analog_stick: &AnalogStick, input_state: &mut InputState) {
    let value;
    let stick;
    let func: fn(&mut AS, f32);

    match analog_stick {
        AnalogStick::LeftXPositive => {
            value = 1.0;
            stick = &mut input_state.left_stick;
            func = AS::set_x_axis;
        }
        AnalogStick::LeftXNegative => {
            value = -1.0;
            stick = &mut input_state.left_stick;
            func = AS::set_x_axis;
        }
        AnalogStick::LeftYPositive => {
            value = 1.0;
            stick = &mut input_state.left_stick;
            func = AS::set_y_axis;
        }
        AnalogStick::LeftYNegative => {
            value = -1.0;
            stick = &mut input_state.left_stick;
            func = AS::set_y_axis;
        }
        AnalogStick::RightXPositive => {
            value = 1.0;
            stick = &mut input_state.right_stick;
            func = AS::set_x_axis;
        }
        AnalogStick::RightXNegative => {
            value = -1.0;
            stick = &mut input_state.right_stick;
            func = AS::set_x_axis;
        }
        AnalogStick::RightYPositive => {
            value = 1.0;
            stick = &mut input_state.right_stick;
            func = AS::set_y_axis;
        }
        AnalogStick::RightYNegative => {
            value = -1.0;
            stick = &mut input_state.right_stick;
            func = AS::set_y_axis;
        }
    }

    func(stick, value)
}
