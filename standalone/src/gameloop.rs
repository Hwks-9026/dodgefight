extern crate core;

use crate::file_loader::load_level;
use crate::game_resources::Rectangle;
use crate::player_simple::PlayerSimple;
use crate::settings::{Keybinds, Settings};
use raylib::consts::KeyboardKey::*;
use raylib::prelude::*;
use raylib_sys::__bool_true_false_are_defined;
use std::cmp::PartialEq;

pub(crate) fn game_loop(mut rl: RaylibHandle, mut thread: RaylibThread, settings: Settings) {
    let mut current_level: i32 = 1;

    let level_data: Vec<Rectangle> = load_level(current_level);

    let mut player_1 = PlayerSimple::new(Vector2::new(130.0, 800.0), 1);
    let mut player_2 = PlayerSimple::new(Vector2::new(130.0, 600.0), 2);

    while !rl.window_should_close() {
        let mut player_1_level_data: Vec<Rectangle> = Vec::with_capacity(level_data.len() + 1);
        let mut player_2_level_data: Vec<Rectangle> = Vec::with_capacity(level_data.len() + 1);

        for rec in level_data.iter() {
            player_1_level_data.push(rec.clone());
            player_2_level_data.push(rec.clone());
        }

        player_1_level_data.push(player_2.hitbox.hitbox.clone());
        player_1.update(&mut rl, &settings, &player_1_level_data);

        player_2_level_data.push(player_1.hitbox.hitbox.clone());
        player_2.update(&mut rl, &settings, &player_2_level_data);

        draw_frame(
            &mut rl,
            &thread,
            &settings,
            &mut player_1,
            &mut player_2,
            &level_data,
        );
    }
}

fn draw_frame(
    rl: &mut RaylibHandle,
    thread: &RaylibThread,
    settings: &Settings,
    player_1: &mut PlayerSimple,
    player_2: &mut PlayerSimple,
    level_data: &Vec<Rectangle>,
) {
    let mut d: RaylibDrawHandle = rl.begin_drawing(&thread);
    d.clear_background(Color::BLACK);

    for r in level_data {
        r.draw(&mut d);
    }

    player_1.draw(&mut d);
    player_2.draw(&mut d);

    if (settings.debug) {
        draw_debug_hud(&mut d, settings, &thread);
    }
}

fn draw_debug_hud(d: &mut RaylibDrawHandle, settings: &Settings, thread: &RaylibThread) {
    let current_time = ((d.get_time() * 100.0).round() / 100.0).to_string();
    d.draw_text(&(current_time), 10, 10, 30, Color::RED);
    d.draw_text(&(d.get_fps().to_string() + " fps"), 10, 40, 30, Color::RED);

    let input_zero = (d.get_screen_width() - 200, d.get_screen_height() - 200);
    d.draw_rectangle(input_zero.0, input_zero.1 + 150, 50, 50, Color::DARKBLUE);
    d.draw_rectangle_lines(input_zero.0, input_zero.1 + 150, 50, 50, Color::DEEPPINK);

    d.draw_rectangle(
        input_zero.0 + 50,
        input_zero.1 + 150,
        50,
        50,
        Color::DARKBLUE,
    );
    d.draw_rectangle_lines(
        input_zero.0 + 50,
        input_zero.1 + 150,
        50,
        50,
        Color::DEEPPINK,
    );

    d.draw_rectangle(
        input_zero.0 + 100,
        input_zero.1 + 150,
        50,
        50,
        Color::DARKBLUE,
    );
    d.draw_rectangle_lines(
        input_zero.0 + 100,
        input_zero.1 + 150,
        50,
        50,
        Color::DEEPPINK,
    );

    d.draw_rectangle(
        input_zero.0 + 50,
        input_zero.1 + 100,
        50,
        50,
        Color::DARKBLUE,
    );
    d.draw_rectangle_lines(
        input_zero.0 + 50,
        input_zero.1 + 100,
        50,
        50,
        Color::DEEPPINK,
    );

    d.draw_rectangle(
        input_zero.0 + 150,
        input_zero.1 + 100,
        50,
        50,
        Color::DARKBLUE,
    );
    d.draw_rectangle_lines(
        input_zero.0 + 150,
        input_zero.1 + 100,
        50,
        50,
        Color::DEEPPINK,
    );

    d.draw_rectangle(
        input_zero.0 + 100,
        input_zero.1 + 150,
        50,
        50,
        Color::DARKBLUE,
    );
    d.draw_rectangle_lines(
        input_zero.0 + 100,
        input_zero.1 + 150,
        50,
        50,
        Color::DEEPPINK,
    );

    let binds: &Keybinds = &settings.keybinds_1;

    let mut color: Color = if d.is_key_down(binds.left) {
        Color::GREEN
    } else {
        Color::RED
    };
    d.draw_text("<", input_zero.0 + 15, input_zero.1 + 150, 50, color);

    color = if d.is_key_down(binds.down) {
        Color::GREEN
    } else {
        Color::RED
    };
    d.draw_text("v", input_zero.0 + 50 + 15, input_zero.1 + 150, 50, color);

    color = if d.is_key_down(binds.right) {
        Color::GREEN
    } else {
        Color::RED
    };
    d.draw_text(">", input_zero.0 + 100 + 15, input_zero.1 + 150, 50, color);

    color = if d.is_key_down(binds.up) {
        Color::GREEN
    } else {
        Color::RED
    };
    d.draw_text("^", input_zero.0 + 50 + 15, input_zero.1 + 100, 50, color);

    color = if d.is_key_down(binds.jump) {
        Color::GREEN
    } else {
        Color::RED
    };
    d.draw_text("j", input_zero.0 + 150 + 15, input_zero.1 + 100, 50, color);
}

/*

       let mut sliding_int: i32 = 0;
       let mut grounded_int: i32 = 0;
       let mut left_right: (i32, i32) = (0, 0);
       for rec in level_data.iter() {
           let cols: (i32, i32) = rec.player_collision(&mut player);
           grounded_int += match cols.0 {
               1 => 1,
               _ => 0
           };
           sliding_int += match cols.0 {
               2 => 2,
               _ => 0
           };
           match cols.1 {
               1 => left_right.0 += 1,
               2 => left_right.1 += 1,
               3 => {left_right.0 +=1; left_right.1 += 1;},
               _ => {}
           }
       }
       println!("{}, {}", grounded_int, sliding_int);
       if(grounded_int >= 1) {player.state = Grounded;}
       else if sliding_int >= 2 {player.state = Sliding;}
       else {if player.state.get_value() == Grounded.get_value() || player.state.get_value() == Sliding.get_value() {player.state = Falling}}
       player.wall_touch = Neither;
       if(left_right.0 != 0 && left_right.1 != 0) {player.wall_touch = Both}
       else if(left_right.0 != 0 && left_right.1 == 0) {player.wall_touch = Left}
       else if(left_right.0 == 0 && left_right.1 != 0) {player.wall_touch = Right}


*/
