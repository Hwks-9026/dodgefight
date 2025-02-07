extern crate core;

mod file_loader;
mod game_resources;
mod player_simple;
mod network_management;

fn main() {
    network_management::start()
}