use raylib::color::Color;

#[derive(Debug)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub color: Color
}
impl Rectangle {
    pub fn new(x: f32, y: f32, width: f32, height: f32, color: Color) -> Rectangle {
        Rectangle {
            x,
            y,
            width,
            height,
            color
        }
    }
}