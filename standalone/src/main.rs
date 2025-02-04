extern crate core;

mod wall;
mod gameloop;
mod file_loader;
mod settings;
mod keycode_conversion;
mod game_resources;
mod player_simple;
mod networking_core;

use std::fs::File;
use std::io::Write;
use std::ops::Add;
use std::path::Path;
use std::str::FromStr;
use raylib::consts::KeyboardKey::*;
use raylib::prelude::*;
use serde_json::{value, Value};
use crate::file_loader::load_settings;
use gameloop::game_loop;
use crate::settings::Settings;
fn main() {
    let settings: Settings = load_settings();
    let (mut rl, thread) = raylib::init()
        .size(settings.target_resolution.0, settings.target_resolution.1)
        .title("rust_game")
        .build();
    rl.set_target_fps(settings.target_fps as u32);
    game_loop(rl, thread, settings);
}