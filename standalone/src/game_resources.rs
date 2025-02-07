use crate::player_simple::PlayerSimple;
use raylib::color::Color;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::math::Vector2;
use raylib_sys::rlSetFramebufferHeight;

pub struct PlayerHitbox {
    pub hitbox: Rectangle,
}

impl PlayerHitbox {
    pub(crate) fn move_x(&mut self, x_displacement: f32) {
        self.hitbox.x += x_displacement;
    }
    pub(crate) fn move_y(&mut self, y_displacement: f32) {
        self.hitbox.y += y_displacement;
    }
}

impl PlayerHitbox {
    pub fn new(x: f32, y: f32, width: f32, height: f32, color: Color) -> PlayerHitbox {
        PlayerHitbox {
            hitbox: Rectangle {
                x: x - (0.5 * width),
                y: (y - height),
                width,
                height,
                color,
            },
        }
    }

    pub fn get_pos(&self) -> Vector2 {
        Vector2::new(
            self.hitbox.x + (self.hitbox.width * 0.5),
            self.hitbox.y + self.hitbox.height,
        )
    }
}

pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub color: Color,
}

impl Rectangle {
    pub(crate) fn clone(&self) -> Rectangle {
        Rectangle {
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
            color: self.color,
        }
    }
}

impl Rectangle {
    pub(crate) fn translate(&mut self, displacement: Vector2) {
        self.x += displacement.x;
        self.y += displacement.y;
    }

    pub fn place_meeting(&self, x: f32, y: f32) -> bool {
        if (self.x > x) {
            return false;
        }
        if (self.y > y) {
            return false;
        }
        if (self.x + self.width < x) {
            return false;
        }
        if (self.y + self.height < y) {
            return false;
        }
        true
    }

    pub fn new(x: f32, y: f32, width: f32, height: f32, color: Color) -> Rectangle {
        Rectangle {
            x,
            y,
            width,
            height,
            color,
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_rectangle(
            self.x as i32,
            self.y as i32,
            self.width as i32,
            self.height as i32,
            self.color,
        );
    }

    pub fn colliding_with(&self, other: &Rectangle) -> bool {
        let a_x = other.x;
        let a_y = other.y;
        let a_w = other.width;
        let a_h = other.height;

        let b_x = self.x;
        let b_y = self.y;
        let b_w = self.width;
        let b_h = self.height;

        if (b_x + b_w < a_x) {
            return false;
        }
        if (b_y + b_h < a_y) {
            return false;
        }
        if (b_x > a_x + a_w) {
            return false;
        }
        if (b_y > a_y + a_h) {
            return false;
        }

        true
    }
}
