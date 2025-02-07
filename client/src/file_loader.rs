use crate::game_resources::Rectangle;
use crate::settings::Keybinds;
use crate::settings::Settings;
use raylib::color::Color;
use serde_json::Value;
const SETTINGS_STRING: &str = include_str!("settings.json");
pub(crate) fn load_settings() -> Settings {
    let v: Value = serde_json::from_str(SETTINGS_STRING).unwrap();
    Settings::new(
        match v["debug"].as_bool() {
            None => false,
            _ => v["debug"].as_bool().unwrap(),
        },
        match v["fullscreen"].as_bool() {
            None => false,
            _ => v["debug"].as_bool().unwrap(),
        },
        (
            &v["target_resolution"]["width"].as_i64(),
            &v["target_resolution"]["height"].as_i64(),
        ),
        &v["target_fps"].as_i64(),
        Keybinds::new(
            &v["keybinds"]["up"].as_i64(),
            &v["keybinds"]["down"].as_i64(),
            &v["keybinds"]["left"].as_i64(),
            &v["keybinds"]["right"].as_i64(),
            &v["keybinds"]["jump"].as_i64(),
            &v["keybinds"]["dash"].as_i64(),
        ),
    )
}

pub(crate) fn load_level(packet_data: &str) -> Vec<Rectangle> {
    let mut level: Vec<Rectangle> = Vec::new();
    let v: Value = serde_json::from_str(packet_data).unwrap();
    let num_hitboxes = v["length"].as_i64().unwrap();
    let mut count: i32 = 1;

    while level.len() < num_hitboxes as usize {
        let p_d = &v[count.to_string()];
        level.push(Rectangle::new(
            p_d["x"].as_f64().unwrap() as f32,
            p_d["y"].as_f64().unwrap() as f32,
            p_d["w"].as_f64().unwrap() as f32,
            p_d["h"].as_f64().unwrap() as f32,
            Color::new(
                p_d["c"]["r"].as_f64().unwrap() as u8,
                p_d["c"]["g"].as_f64().unwrap() as u8,
                p_d["c"]["b"].as_f64().unwrap() as u8,
                p_d["c"]["a"].as_f64().unwrap() as u8,
            ),
        ));
        count += 1;
    }
    level
}
