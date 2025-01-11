use paste::paste;

use crate::console::input::{ButtonCode, InputState, PlayerInputEntry, MOUSE_INVALID_BIT};
use crate::console::WasmContexts;

use wasmtime::{Caller, Linker};

#[derive(Clone)]
pub struct InputContext {
    pub(crate) input_entries: Box<[PlayerInputEntry]>,
    pub(crate) mouse_locked: bool,
}

impl InputContext {
    pub fn new(num_players: usize) -> Self {
        Self {
            input_entries: (0..num_players)
                .map(|_| PlayerInputEntry::default())
                .collect(),
            mouse_locked: false,
        }
    }
}

macro_rules! derive_generate_input_api {
    (
        Buttons { $($btn_name:ident: $btn_code:ident,)* },
        Analogs { $($anlg_name:ident,)* },
        Triggers { $($trg_name:ident,)* },
        Mouse {
            Buttons { $($mbtn_name:ident,)* },
            Axis { $($maxis_name:ident,)* },
            Wheel { $($mwheel_name:ident,)* },
         },
    ) => {
        paste! {
            impl InputContext {
                pub fn link(linker: &mut Linker<WasmContexts>) {
                    $(linker
                        .func_wrap("env", stringify!([<button_ $btn_name _pressed>]), |caller: Caller<WasmContexts>, p: i32| {
                            caller.data().input.[<button_ $btn_name _pressed>](p)
                        })
                        .unwrap();

                    linker
                        .func_wrap("env", stringify!([<button_ $btn_name _released>]), |caller: Caller<WasmContexts>, p: i32| {
                            caller.data().input.[<button_ $btn_name _released>](p)
                        })
                        .unwrap();

                    linker
                        .func_wrap("env", stringify!([<button_ $btn_name _held>]), |caller: Caller<WasmContexts>, p: i32| {
                            caller.data().input.[<button_ $btn_name _held>](p)
                        })
                        .unwrap();
                    )*

                    $(linker
                        .func_wrap("env", stringify!([<analog_ $anlg_name _x>]), |caller: Caller<WasmContexts>, p: i32| {
                            caller.data().input.[<analog_ $anlg_name _x>](p)
                        })
                        .unwrap();

                    linker
                        .func_wrap("env", stringify!([<analog_ $anlg_name _y>]), |caller: Caller<WasmContexts>, p: i32| {
                            caller.data().input.[<analog_ $anlg_name _y>](p)
                        })
                        .unwrap();
                    )*

                    $(
                        linker
                        .func_wrap("env", stringify!([<trigger_ $trg_name>]), |caller: Caller<WasmContexts>, p: i32| {
                            caller.data().input.[<trigger_ $trg_name>](p)
                        })
                        .unwrap();
                    )*

                    $(
                        linker
                        .func_wrap("env", stringify!([<mouse_ $mbtn_name _pressed>]), |caller: Caller<WasmContexts>, p: i32| {
                            caller.data().input.[<mouse_ $mbtn_name _pressed>](p)
                        })
                        .unwrap();

                        linker
                        .func_wrap("env", stringify!([<mouse_ $mbtn_name _released>]), |caller: Caller<WasmContexts>, p: i32| {
                            caller.data().input.[<mouse_ $mbtn_name _released>](p)
                        })
                        .unwrap();

                        linker
                        .func_wrap("env", stringify!([<mouse_ $mbtn_name _held>]), |caller: Caller<WasmContexts>, p: i32| {
                            caller.data().input.[<mouse_ $mbtn_name _held>](p)
                        })
                        .unwrap();
                    )*

                    $(
                        linker
                        .func_wrap("env", stringify!([<mouse_ $maxis_name _pos>]), |caller: Caller<WasmContexts>, p: i32| {
                            caller.data().input.[<mouse_ $maxis_name _pos>](p)
                        })
                        .unwrap();

                        linker
                        .func_wrap("env", stringify!([<mouse_ $maxis_name _delta>]), |caller: Caller<WasmContexts>, p: i32| {
                            caller.data().input.[<mouse_ $maxis_name _delta>](p)
                        })
                        .unwrap();
                    )*

                    $(
                        linker
                        .func_wrap("env", stringify!([<mouse_wheel_ $mwheel_name>]), |caller: Caller<WasmContexts>, p: i32| {
                            caller.data().input.[<mouse_wheel_ $mwheel_name>](p)
                        })
                        .unwrap();
                    )*

                    linker
                    .func_wrap("env", "raw_mouse_state", |caller: Caller<WasmContexts>, p: i32| {
                        caller.data().input.raw_mouse_state(p)
                    })
                    .unwrap();

                    linker
                    .func_wrap("env", "raw_input_state", |caller: Caller<WasmContexts>, p: i32| {
                        caller.data().input.raw_input_state(p)
                    })
                    .unwrap();

                    linker
                    .func_wrap("env", "lock_mouse", |mut caller: Caller<WasmContexts>, p: i32| {
                        caller.data_mut().input.lock_mouse(p)
                    })
                    .unwrap();
                }

                $(
                    fn [<button_ $btn_name _pressed>](&self, player_id: i32) -> i32 {
                        if let Some(player_input) = &self.input_entries.get(player_id as usize) {
                            let prev = player_input.previous.get_button_state(ButtonCode::$btn_code);
                            let curr = player_input.current.buttons.get_button_state(ButtonCode::$btn_code);
                            (prev == false && curr == true) as i32
                        } else {
                            -1
                        }
                    }

                    fn [<button_ $btn_name _released>](&self, player_id: i32) -> i32 {
                        if let Some(player_input) = &self.input_entries.get(player_id as usize) {
                            let prev = player_input.previous.get_button_state(ButtonCode::$btn_code);
                            let curr = player_input.current.buttons.get_button_state(ButtonCode::$btn_code);
                            (prev == true && curr == false) as i32
                        } else {
                            -1
                        }
                    }

                    fn [<button_ $btn_name _held>](&self, player_id: i32) -> i32 {
                        if let Some(player_input) = &self.input_entries.get(player_id as usize) {
                            player_input.current.buttons.get_button_state(ButtonCode::$btn_code) as i32
                        } else {
                            -1
                        }
                    }
                )*

                $(
                    fn [<analog_ $anlg_name _x>](&self, player_id: i32) -> f32 {
                        if let Some(player_input) = &self.input_entries.get(player_id as usize) {
                            player_input.current.[<$anlg_name _stick>].get_x_axis()
                        } else {
                            f32::NAN
                        }
                    }

                    fn [<analog_ $anlg_name _y>](&self, player_id: i32) -> f32 {
                        if let Some(player_input) = &self.input_entries.get(player_id as usize) {
                            player_input.current.[<$anlg_name _stick>].get_y_axis()
                        } else {
                            f32::NAN
                        }
                    }
                )*

                $(
                    fn [<trigger_ $trg_name>](&self, player_id: i32) -> f32 {
                        if let Some(player_input) = &self.input_entries.get(player_id as usize) {
                            player_input.current.[<$trg_name _trigger>].get_value()
                        } else {
                            f32::NAN
                        }
                    }
                )*

                $(
                    fn [<mouse_ $mbtn_name _pressed>](&self, player_id: i32) -> i32 {
                        if let Some(player_input) = &self.input_entries.get(player_id as usize) {
                            let prev = player_input.previous_mouse.[<get_ $mbtn_name _button_down>]();
                            let curr = player_input.current_mouse.[<get_ $mbtn_name _button_down>]();
                            (prev == false && curr == true) as i32
                        } else {
                            -1
                        }
                    }

                    fn [<mouse_ $mbtn_name _released>](&self, player_id: i32) -> i32 {
                        if let Some(player_input) = &self.input_entries.get(player_id as usize) {
                            let prev = player_input.previous_mouse.[<get_ $mbtn_name _button_down>]();
                            let curr = player_input.current_mouse.[<get_ $mbtn_name _button_down>]();
                            (prev == true && curr == false) as i32
                        } else {
                            -1
                        }
                    }

                    fn [<mouse_ $mbtn_name _held>](&self, player_id: i32) -> i32 {
                        if let Some(player_input) = &self.input_entries.get(player_id as usize) {
                            player_input.current_mouse.[<get_ $mbtn_name _button_down>]() as i32
                        } else {
                            -1
                        }
                    }
                )*

                $(
                    fn [<mouse_ $maxis_name _pos>](&self, player_id: i32) -> i32 {
                        if let Some(player_input) = &self.input_entries.get(player_id as usize) {
                            player_input.current_mouse.[<get_ $maxis_name _pos>]() as i32
                        } else {
                            -1
                        }
                    }

                    fn [<mouse_ $maxis_name _delta>](&self, player_id: i32) -> i32 {
                        if let Some(player_input) = &self.input_entries.get(player_id as usize) {
                            player_input.current_mouse.[<get_ $maxis_name _delta>]() as i32
                        } else {
                            i32::MIN
                        }
                    }
                )*

                $(
                    fn [<mouse_wheel_ $mwheel_name>](&self, player_id: i32) -> i32 {
                        if let Some(player_input) = &self.input_entries.get(player_id as usize) {
                            player_input.current_mouse.[<get_wheel_ $mwheel_name>]() as i32
                        } else {
                            -1
                        }
                    }
                )*

                fn raw_mouse_state(&self, player_id: i32) -> i64 {
                    if let Some(player_input) = self.input_entries.get(player_id as usize) {
                        player_input.current_mouse.0 as i64
                    } else {
                        1 << MOUSE_INVALID_BIT
                    }
                }

                fn raw_input_state(&self, player_id: i32) -> i64 {
                    let state = if let Some(player_input) = self.input_entries.get(player_id as usize) {
                        player_input.current
                    } else {
                        InputState::INVALID_STATE
                    };

                    unsafe { std::mem::transmute(state) }
                }

                fn lock_mouse(&mut self, locked: i32) {
                    if locked != 0 {
                        self.mouse_locked = true
                    } else {
                        self.mouse_locked = false
                    };
                }
            }
        }
    }
}

derive_generate_input_api! {
    Buttons {
        a: ButtonA,
        b: ButtonB,
        c: ButtonC,
        d: ButtonD,
        up: Up,
        down: Down,
        left: Left,
        right: Right,
        start: Start,
        select: Select,
        left_shoulder: LeftShoulder,
        right_shoulder: RightShoulder,
        left_stick: LeftStick,
        right_stick: RightStick,
        left_trigger: LeftTrigger,
        right_trigger: RightTrigger,
    },
    Analogs {
        left,
        right,
    },
    Triggers {
        left,
        right,
    },
    Mouse {
        Buttons {
            left,
            right,
            middle,
        },
        Axis {
            x,
            y,
        },
        Wheel {
            up,
            down,
            left,
            right,
        },
    },
}
