use serde_json::Value;
use raylib::color::Color;
use crate::game_resources::Rectangle;

const LEVEL_1: &str = include_str!("level1.json");

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

