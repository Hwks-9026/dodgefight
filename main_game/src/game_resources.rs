use raylib::color::Color;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::math::Vector2;
use crate::player::Player;
use crate::player::State::Jumping;

pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub color: Color
}

impl Rectangle {
    pub(crate) fn translate(&mut self, displacement: Vector2) {
        self.x += displacement.x;
        self.y += displacement.y;
    }
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

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_rectangle(self.x as i32, self.y as i32, self.width as i32, self.height as i32, self.color);
    }

    fn colliding_with(&self, other: &Rectangle) -> bool {
        let a_x = other.x;
        let a_y = other.y;
        let a_w = other.width;
        let a_h = other.height;

        let b_x = self.x;
        let b_y = self.y;
        let b_w = self.width;
        let b_h = self.height;

        if(b_x + b_w < a_x){return false}
        if(b_y + b_h < a_y){return false}
        if(b_x > a_x + a_w){return false}
        if(b_y > a_y + a_h){return false}

        true
    }

    pub fn player_collision(&self, player: &mut Player) -> (i32, i32) {
        let mut p0 = &mut player.hitbox;
        let mut player_hitboxes: Vec<Rectangle> = Vec::new();

        let gap_const: f32 = 5.0;
        let thickness: f32 = 1.0;

        player_hitboxes.push(Rectangle::new(p0.x + gap_const, p0.y + p0.height, p0.width - gap_const, thickness, Color::new(0, 0, 0, 0)));
        player_hitboxes.push(Rectangle::new(p0.x + gap_const, p0.y - thickness, p0.width - gap_const, thickness, Color::new(0, 0, 0, 0)));
        player_hitboxes.push(Rectangle::new(p0.x - thickness, p0.y + gap_const, thickness, p0.height - gap_const, Color::new(0, 0, 0, 0)));
        player_hitboxes.push(Rectangle::new(p0.x + p0.width, p0.y + gap_const, thickness, p0.height - gap_const, Color::new(0, 0, 0, 0)));

        let mut touching: [bool; 4] = [false; 4]; //in the order: Bottom, Top, Left, Right
        let mut count = 0;
        for hb in &player_hitboxes {
            if hb.colliding_with(self) {
                touching[count] = true;
            }
            count += 1;
        }
        let mut state = (0, 0);
        if(touching[0]) {if player.state.get_value() != Jumping.get_value() {state.0 = 1}}
        if(touching[1]) {player.vel.y = 0.0;}
        if(touching[2]) {state.1 += 1}
        if(touching[3]) {state.1 += 2}

        if(state.1 > 0) {
            let mid = (p0.x + (p0.width/2.0));
            if !(mid > self.x && mid < self.x + self.width) && state.1 > 0 {
                state.0 = 2
            }
        }

        state
    }
}