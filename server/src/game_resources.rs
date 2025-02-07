use raylib::color::Color;
use raylib::math::Vector2;
#[derive(Clone, Debug)]
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
            hitbox:
                Rectangle {
                x: x - (0.5 * width),
                y: (y - height),
                width,
                height,
                color
                }
        }
    }

    pub fn get_pos(&self) -> Vector2 {
        Vector2::new(self.hitbox.x + (self.hitbox.width * 0.5), self.hitbox.y + self.hitbox.height)
    }
}
#[derive(Clone, Debug)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub color: Color
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
    pub fn place_meeting(&self, x: f32, y: f32) -> bool {
        if self.x > x { return false; }
        if self.y > y { return false; }
        if self.x + self.width < x { return false; }
        if self.y + self.height < y { return false; }
        true
    }

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
