use raylib::color::Color;
use raylib::drawing::RaylibDrawHandle;
use raylib::RaylibHandle;
use raylib::math::Vector2;
use raylib_sys::KeyboardKey::*;
use crate::game_resources::{PlayerHitbox, Rectangle};
const SUBPIXEL: f32 = 0.5;
const BUFFERTIME: i32 = 15;
const MAXJUMPS: i32 = 2;
const JUMPHOLDFRAMES: i32 = 16;
const COYOTEHANGFRAMES: i32 = 2;
const COYOTEJUMPFRAMES: i32 = 16;
pub struct PlayerSimple {
    which_player: i32,
    health: f64,
    //movement
    move_dir: i32,
    last_move_dir: i32,
    move_speed: f32,
    x_speed: f32,
    y_speed: f32,

    //jumping
    gravity: f32,
    terminal_velocity: f32,
    jump_key_buffered: bool,
    jump_key_timer: i32,

    //double jumps
    jump_count: i32,
    jump_hold_timer: i32,
    on_ground: bool,

    //wall jumps
    can_wall_jump: i32,
    wall_jump_coyote_timer: i32,

    //coyote time
    coyote_hang_timer: i32,
    coyote_jump_timer: i32,


    pub(crate) hitbox: PlayerHitbox,


}

impl PlayerSimple {
    pub fn new(starting_pos: Vector2, player_number: i32) -> PlayerSimple {
        PlayerSimple {
            which_player: player_number,
            health: 1200.0,
            move_dir: 0,
            last_move_dir: 1,
            move_speed: 7.0,
            x_speed: 0.0,
            y_speed: 0.0,
            gravity: 0.275,
            terminal_velocity: 12.0,
            jump_key_buffered: false,
            jump_key_timer: 0,
            jump_count: 2,
            jump_hold_timer: 0,
            on_ground: false,
            can_wall_jump: 0,
            wall_jump_coyote_timer: 0,
            coyote_hang_timer: 0,
            coyote_jump_timer: 0,
            hitbox: PlayerHitbox::new(starting_pos.x, starting_pos.y, 50.0, 100.0, Color::RED)
        }
    }

    pub fn update(&mut self, inputs: [i32; 4], level_data: &Vec<Rectangle>) {

        if(self.hitbox.hitbox.y > 1500.0 ) {
            self.hitbox.hitbox.x = 130.0;
            self.hitbox.hitbox.y = 800.0;
        }

        let left_key = inputs[0];
        let right_key = inputs[1];
        let jump_key_pressed = match inputs[2] {1 => true, _ => false};
        let jump_key = match inputs[3] {1 => true, _ => false};

        self.move_dir = right_key - left_key;
        if self.move_dir != 0 {
            self.last_move_dir = self.move_dir;
        }
        if jump_key_pressed {
            self.jump_key_timer = BUFFERTIME;
        }
        if self.jump_key_timer > 0 {
            self.jump_key_buffered = true;
            self.jump_key_timer -= 1;
        }
        else {
            self.jump_key_buffered = false;
        }

        self.x_update(level_data);
        self.y_update(level_data, jump_key);

        if(self.jump_count > 1) {self.hitbox.hitbox.color = Color::YELLOW;}
        else { self.hitbox.hitbox.color = Color::RED; }

    }

    fn x_update(&mut self, level_data: &Vec<Rectangle>) {
        if(self.x_speed.abs() < self.move_speed.abs()) {
            self.x_speed = self.move_dir as f32 * 0.1 * self.move_speed + 0.95 * self.x_speed;
        }
        else if (self.move_dir != 0){
            self.x_speed = self.move_dir as f32 * self.move_speed;
        }
        else {
            self.x_speed = 0.5 * self.x_speed;
        }


        let mut pos: Vector2 = self.hitbox.get_pos();

        //X Collision:
        if self.colliding(pos.x + self.x_speed, pos.y, level_data) {

            let pixel_check = SUBPIXEL * self.x_speed.signum();

            while !(self.colliding(pos.x + pixel_check, pos.y, level_data)) {
                self.hitbox.move_x(pixel_check);
                pos = self.hitbox.get_pos();
            }
            self.can_wall_jump = self.x_speed.signum() as i32;
            self.wall_jump_coyote_timer = COYOTEJUMPFRAMES;

            self.x_speed = 0.0;
        }
        else {
            println!("{}", self.wall_jump_coyote_timer);
            if self.wall_jump_coyote_timer > 0 {
                self.wall_jump_coyote_timer -= 1;
            }
            else {
                self.can_wall_jump = 0;
            }
        }
        self.hitbox.move_x(self.x_speed);
    }

    fn y_update(&mut self, level_data: &Vec<Rectangle>, jump_key: bool) {
        //Y Movement
        let mut pos: Vector2 = self.hitbox.get_pos();
        if self.coyote_hang_timer < COYOTEHANGFRAMES {
            self.coyote_hang_timer += 1;
        }
        else {
            self.y_speed += self.gravity;
            self.set_on_ground(false)
        }


        if (self.on_ground) {
            self.jump_count = 0;
            self.coyote_jump_timer = COYOTEJUMPFRAMES;
        }
        else {
            self.coyote_jump_timer -= 1;
            if self.coyote_jump_timer == 0 {
                self.jump_count = 1;
            }
        }

        if(self.y_speed > self.terminal_velocity) { self.y_speed = self.terminal_velocity; }

        //Initiate Jump
        if (self.jump_key_buffered && (self.jump_count < MAXJUMPS || self.can_wall_jump != 0)) {
            self.jump_key_buffered = false;
            self.jump_key_timer = 0;



            self.jump_hold_timer = JUMPHOLDFRAMES;
            self.set_on_ground(false);
            println!("{}", self.can_wall_jump);
            match self.can_wall_jump {
                1 => self.x_speed += 100.0,
                -1 => self.x_speed -= 100.0,
                _ => {self.jump_count += 1;}
            }
        }

        if !(jump_key) {
            self.jump_hold_timer = 0;
        }
        if(self.jump_hold_timer > 0) {
            self.jump_hold_timer -= 1;
            self.y_speed = -1.0 * compute_jump_height(self.jump_hold_timer);
        }


        if self.colliding(pos.x, pos.y + self.y_speed, level_data) {

            let pixel_check = SUBPIXEL * self.y_speed.signum();

            while !(self.colliding(pos.x, pos.y + pixel_check, level_data)) {
                self.hitbox.move_y(pixel_check);
                pos = self.hitbox.get_pos();
            }

            self.y_speed = 0.0;
        };
        if self.colliding(pos.x, pos.y + 1.0, level_data) && self.y_speed >= 0.0 {
           self.set_on_ground(true)
        }

        self.hitbox.move_y(self.y_speed);
    }

    fn set_on_ground(&mut self, value: bool) {
        if value {
            self.on_ground = true;
            self.coyote_hang_timer = 0;
        }
        else {
            self.on_ground = false;
        }
    }

    fn colliding(&self, x: f32, y: f32, level_data: &Vec<Rectangle>) -> bool {
        for rect in level_data.iter() {
            let mut test_y = y;
            let mut test_x = x;
            if rect.y + rect.height < y {
                test_y = y - self.hitbox.hitbox.height;
                if(test_y < rect.y + rect.height) {
                    test_y = rect.y + (rect.height/2.0);
                }
            }


            if rect.x > test_x && rect.x < test_x + self.hitbox.hitbox.width/2.0 {
                test_x = rect.x + 1.0;
            }
            if rect.x + rect.width < test_x && rect.x + rect.width > test_x - self.hitbox.hitbox.width/2.0 {
                test_x = rect.x + rect.width - 1.0;
            }


            if rect.place_meeting(test_x, test_y) {return true}
        }
        false
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        self.hitbox.hitbox.draw(d)
    }
}



fn to_1_0(b: bool) -> i32 {
    match b {
        true => 1,
        false => 0,
    }
}

fn compute_jump_height(current_frame: i32) -> f32 {
    let x = (JUMPHOLDFRAMES - current_frame) as f32;
    25.0 * (1.0 / (1.0 + (-0.2 * x).exp()) - 0.5)

    /*
    let term_1: f32 = 2.0 / (1.0 + (-0.6*(x - 1.0)).exp());
    return 5.0 * (term_1);
     */
}