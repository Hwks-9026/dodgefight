extern crate core;
mod file_loader;
mod game_resources;
mod gameloop;
mod keycode_conversion;
mod settings;

use std::env;
use crate::file_loader::load_settings;
use crate::gameloop::game_loop;
use crate::settings::Settings;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {return}

    let settings: Settings = load_settings();
    let (mut rl, thread) = raylib::init()
        .size(settings.target_resolution.0, settings.target_resolution.1)
        .title("rust_game")
        .build();
    rl.set_target_fps(settings.target_fps as u32);
    game_loop(rl, thread, settings, args[1].clone());
}
