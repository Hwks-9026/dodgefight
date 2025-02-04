use serde_json::{Result, Value};
use std::fs::File;
use raylib::color::Color;
use crate::game_resources::Rectangle;
use crate::settings::Settings;
use crate::settings::Keybinds;
const SETTINGS_STRING: &str = include_str!("settings.json");
const LEVEL_1: &str = include_str!("level1.json");
pub(crate) fn load_settings() -> Settings {
    let v: Value = serde_json::from_str(SETTINGS_STRING).unwrap();
    Settings::new(match v["debug"].as_bool() {None => false, _ => v["debug"].as_bool().unwrap()}, match v["fullscreen"].as_bool() {None => false, _ => v["debug"].as_bool().unwrap()}, (&v["target_resolution"]["width"].as_i64(), &v["target_resolution"]["height"].as_i64()), &v["target_fps"].as_i64(),
                  Keybinds::new(&v["keybinds_1"]["up"].as_i64(), &v["keybinds_1"]["down"].as_i64(), &v["keybinds_1"]["left"].as_i64(), &v["keybinds_1"]["right"].as_i64(), &v["keybinds_1"]["jump"].as_i64(), &v["keybinds_1"]["dash"].as_i64()),
                  Keybinds::new(&v["keybinds_2"]["up"].as_i64(), &v["keybinds_2"]["down"].as_i64(), &v["keybinds_2"]["left"].as_i64(), &v["keybinds_2"]["right"].as_i64(), &v["keybinds_2"]["jump"].as_i64(), &v["keybinds_2"]["dash"].as_i64())
    )
}

pub(crate) fn load_level(level_id: i32) -> Vec<Rectangle>{
    let mut level: Vec<Rectangle> = Vec::new();
    let level_path = match level_id {
        1 => LEVEL_1,
        _ => LEVEL_1,
    };
    let v: Value = serde_json::from_str(level_path).unwrap();
    let num_hitboxes = v["length"].as_i64().unwrap();
    let mut count: i32 = 1;
    while level.len() < num_hitboxes as usize {
        level.push(Rectangle::new(v[count.to_string()]["x"].as_f64().unwrap() as f32,
                                  v[count.to_string()]["y"].as_f64().unwrap() as f32,
                                  v[count.to_string()]["w"].as_f64().unwrap() as f32,
                                  v[count.to_string()]["h"].as_f64().unwrap() as f32,
                                    Color::TEAL));
        count += 1;
    }
    return level;
}

