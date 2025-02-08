extern crate core;

mod file_loader;
mod game_resources;
mod gameloop;
mod keycode_conversion;
mod networking_core;
mod player_simple;
mod settings;
mod wall;

use crate::file_loader::load_settings;
use crate::settings::Settings;
use gameloop::game_loop;
use raylib::consts::KeyboardKey::*;
use raylib::prelude::*;
use std::io::Write;
use std::ops::Add;
use std::str::FromStr;
fn main() {
    let settings: Settings = load_settings();
    let (mut rl, thread) = raylib::init()
        .size(settings.target_resolution.0, settings.target_resolution.1)
        .title("rust_game")
        .build();
    rl.set_target_fps(settings.target_fps as u32);
    game_loop(rl, thread, settings);
}
