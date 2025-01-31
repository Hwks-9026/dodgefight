use std::cmp::PartialEq;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::ease::quad_in;
use raylib::math::Vector2;
use raylib::prelude::Color;
use raylib::RaylibHandle;
use raylib_sys::KeyboardKey::KEY_DOWN;
use crate::game_resources::Rectangle;
use crate::settings::{Keybinds, Settings};

const PLAYER_SPEED: f32 = 300.0;
const PLAYER_WIDTH: i32 = 30;
const PLAYER_HEIGHT: i32 = 50;

const TIME_SCALE: f32 = 0.50;
#[derive(Debug)]
pub enum State {
    Grounded = 0,
    Falling = 1,
    Dashing = 2,
    Knockback = 3,
    Noclip = 4,
    Sliding = 5,
    Jumping = 6,
}

impl State {
    pub fn get_value(&self) -> i32 {
        match self {
            State::Grounded => 0,
            State::Falling => 1,
            State::Dashing => 2,
            State::Knockback => 3,
            State::Noclip => 4,
            State::Sliding => 5,
            State::Jumping => 6,
        }
    }
}


#[derive(Debug)]
pub enum WallsTouching {
    Neither = 0,
    Left = 1,
    Right = 2,
    Both = 3,
}

impl WallsTouching {
    pub(crate) fn get_value(&self) -> i32 {
        match self {
            WallsTouching::Left => 1,
            WallsTouching::Right => 2,
            WallsTouching::Both => 3,
            _ => 0
        }


    }
}

pub struct Player {
    pub state: State,
    pub vel: Vector2,
    pub hitbox: Rectangle,
    pub wall_touch: WallsTouching,
    jump_frames: u32,
}



impl Player {
    pub fn new(x: i32, y: i32) -> Player {
        Player {
            state: State::Falling,
            vel: Vector2::zero(),
            hitbox: Rectangle::new((x - PLAYER_WIDTH / 2) as f32, (y - PLAYER_HEIGHT) as f32, PLAYER_WIDTH as f32, PLAYER_HEIGHT as f32, Color::WHITE),
            wall_touch: WallsTouching::Neither,
            jump_frames: 0,
        }
    }

    pub fn update(&mut self, rl: &RaylibHandle, settings: &Settings, dt: f32) {
        match self.state {
            State::Falling => self.update_falling(rl, settings, dt),
            State::Grounded => self.update_grounded(rl, settings, dt),
            State::Noclip => self.update_noclip(rl, settings, dt),
            State::Sliding => self.update_sliding(rl, settings, dt),
            State::Jumping => self.update_jumping(rl, settings, dt),
            _ => {}
        }

    }

    fn update_jumping(&mut self, rl: &RaylibHandle, settings: &Settings, dt: f32) {
        if !(rl.is_key_down(settings.keybinds.jump) && self.jump_frames < (rl.get_fps() / 2)) {
            self.state = State::Falling;
            self.update_falling(rl, settings, dt);
            return;
        }
        self.vel = Vector2::zero();
        let move_vector: Vector2 = Vector2::new(self.create_input_vector(rl, settings).x, 0.0);
        self.vel += move_vector * PLAYER_SPEED;
        self.vel += Vector2::new(0.0, -1000.0);
        self.update_hitbox(self.vel * dt * TIME_SCALE);
        self.jump_frames += 1;
    }

    fn update_sliding(&mut self, rl: &RaylibHandle, settings: &Settings, dt: f32) {
        self.vel = Vector2::zero();
        let move_vector: Vector2 = Vector2::new(self.create_input_vector(rl, settings).x, 0.0);
        self.vel += move_vector * PLAYER_SPEED;
        self.vel += Vector2::new(0.0, 200.0);
        self.update_hitbox(self.vel * dt * TIME_SCALE)
    }

    fn update_noclip(&mut self, rl: &RaylibHandle, settings: &Settings, dt: f32) {
        self.vel = Vector2::zero();
        let move_vector: Vector2 = self.create_input_vector(rl, settings);
        self.vel += move_vector * PLAYER_SPEED;
    }

    fn update_falling(&mut self, rl: &RaylibHandle, settings: &Settings, dt: f32) {
        self.vel = Vector2::zero();
        let move_vector: Vector2 = Vector2::new(self.create_input_vector(rl, settings).x, 0.0);
        self.vel += move_vector * PLAYER_SPEED;
        self.vel += Vector2::new(0.0, 500.0);
        self.update_hitbox(self.vel * dt * TIME_SCALE)
    }

    fn update_grounded(&mut self, rl: &RaylibHandle, settings: &Settings, dt: f32) {
        self.vel = Vector2::zero();
        let move_vector: Vector2 = Vector2::new(self.create_input_vector(rl, settings).x, 0.0);
        self.vel += move_vector * PLAYER_SPEED;
        self.update_hitbox(self.vel * dt * TIME_SCALE);

        if(rl.is_key_down(settings.keybinds.jump)) {
            self.state = State::Jumping;
        }
    }


    fn update_hitbox(&mut self, displacement: Vector2) {
        self.hitbox.translate(displacement);
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        self.hitbox.draw(d);
    }
    pub fn create_input_vector(&mut self, rl: &RaylibHandle, settings: &Settings) -> Vector2 {
        let mut move_vector: Vector2 = Vector2::new(0.0, 0.0,);
        let binds: &Keybinds = &settings.keybinds;

        let wt = self.wall_touch.get_value();
        println!("wall_touch: {:?}. State: {:?}", wt, self.state);
        if(wt < 3) {
            if (rl.is_key_down(binds.right) && wt != 2) { move_vector.x += 1.0; }
            if (rl.is_key_down(binds.left) && wt != 1) { move_vector.x -= 1.0; }
        }
        if(rl.is_key_down(binds.up)) { move_vector.y -= 1.0; }
        if(rl.is_key_down(binds.down)) { move_vector.y += 1.0; }
        move_vector

    }
}

