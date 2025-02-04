use raylib_sys::KeyboardKey;
use crate::keycode_conversion::code_to_key;

pub(crate) struct Settings {
    pub(crate) debug: bool,
    pub(crate) fullscreen: bool,
    pub(crate) target_resolution: (i32, i32),
    pub(crate) target_fps: i32,
    pub(crate) keybinds_1: Keybinds,
    pub(crate) keybinds_2: Keybinds,
}
impl Settings {
    pub(crate) fn new(debug: bool, fullscreen: bool, target_resolution: (&Option<i64>, &Option<i64>), target_fps: &Option<i64>, keybinds_1: Keybinds, keybinds_2: Keybinds) -> Self {
        let (x, y) = target_resolution;

        Settings {
            debug,
            fullscreen,
            target_resolution: (option_i64_to_i32(x), option_i64_to_i32(y)),
            target_fps: option_i64_to_i32(target_fps),
            keybinds_1,
            keybinds_2
        }
    }

}

pub(crate) struct Keybinds {
    pub(crate) up: KeyboardKey,
    pub(crate) down: KeyboardKey,
    pub(crate) left: KeyboardKey,
    pub(crate) right: KeyboardKey,
    pub(crate) jump: KeyboardKey,
    pub(crate) dash: KeyboardKey
}


impl Keybinds {
    pub(crate) fn new(up: &Option<i64>, down: &Option<i64>, left: &Option<i64>, right: &Option<i64>, jump: &Option<i64>, dash: &Option<i64>) -> Self {

        Keybinds {
            up: code_to_key(option_i64_to_i32(up)).unwrap(),
            down: code_to_key(option_i64_to_i32(down)).unwrap(),
            left: code_to_key(option_i64_to_i32(left)).unwrap(),
            right: code_to_key(option_i64_to_i32(right)).unwrap(),
            jump: code_to_key(option_i64_to_i32(jump)).unwrap(),
            dash: code_to_key(option_i64_to_i32(dash)).unwrap()
        }
    }
}

fn option_i64_to_i32(value: &Option<i64>) -> i32 {
    match value {None => 0, _ => value.unwrap() as i32}
}