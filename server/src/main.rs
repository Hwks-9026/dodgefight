extern crate core;

mod file_loader;
mod game_resources;
mod network_management;
mod player_simple;

fn main() {
    network_management::start()
}
