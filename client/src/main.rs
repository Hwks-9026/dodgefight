extern crate core;
mod keycode_conversion;
mod gameloop;
mod file_loader;
mod game_resources;
mod settings;

use std::fs::File;
use std::io::Write;
use std::ops::Add;
use std::path::Path;
use std::str::FromStr;
use crate::file_loader::load_settings;
use crate::gameloop::game_loop;
use crate::settings::Settings;

fn main() {
    let settings: Settings = load_settings();
    let (mut rl, thread) = raylib::init()
        .size(settings.target_resolution.0, settings.target_resolution.1)
        .title("rust_game")
        .build();
    rl.set_target_fps(settings.target_fps as u32);
    unsafe { game_loop(rl, thread, settings); }

}
