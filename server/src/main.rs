extern crate core;

mod file_loader;
mod game_resources;
mod player_simple;
mod network_management;

use std::io::Write;
use std::ops::Add;
use std::str::FromStr;
use raylib::consts::KeyboardKey::*;
use raylib::prelude::*;

fn main() {
    network_management::start()
}