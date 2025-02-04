extern crate core;

use std::cmp::PartialEq;
use std::io::{Read, Write};
use std::net::TcpStream;
use raylib::consts::KeyboardKey::*;
use raylib::prelude::*;
use raylib_sys::__bool_true_false_are_defined;
use crate::file_loader::load_level;
use crate::settings::{Keybinds, Settings};
use crate::game_resources::Rectangle;


pub(crate) unsafe fn game_loop(mut rl: RaylibHandle, mut thread: RaylibThread, settings: Settings) {
    let binds = &settings.keybinds;

    while !rl.window_should_close() {

        let mut packet: String = "".to_string();
        packet += &*to_1_0(rl.is_key_down(binds.left)).to_string();
        packet += &*to_1_0(rl.is_key_down(binds.right)).to_string();
        packet += &*to_1_0(rl.is_key_pressed(binds.jump)).to_string();
        packet += &*to_1_0(rl.is_key_down(binds.jump)).to_string();

        let mut stream: TcpStream = TcpStream::connect("127.0.0.1:9999").expect("Could not connect to server");
        _ = stream.write(packet.as_bytes());
        _ = stream.flush().expect("Could not flush stream");
        let mut read_buffer = [0; 4096];
        stream.read(&mut read_buffer).expect("Could not read from server");
        let result: String = String::from_utf8_lossy(&*Vec::from(&read_buffer[..])).trim_end_matches(char::from(0)).to_string();




        draw_frame(&mut rl, &thread, &settings, &load_level(result));
    }
}

fn draw_frame(rl: &mut RaylibHandle, thread: &RaylibThread, settings: &Settings, level_data: &Vec<Rectangle>) {
    let mut d: RaylibDrawHandle = rl.begin_drawing(&thread);
    d.clear_background(Color::BLACK);

    for r in level_data {
        r.draw(&mut d);
    }

    if(settings.debug) {draw_debug_hud(&mut d, settings, &thread);}
}

fn draw_debug_hud(d: &mut RaylibDrawHandle, settings: &Settings, thread: &RaylibThread) {
    let current_time = ((d.get_time() * 100.0).round()/100.0).to_string();
    d.draw_text(&(current_time), 10, 10, 30, Color::RED);
    d.draw_text(&(d.get_fps().to_string() + " fps"), 10, 40, 30, Color::RED);

    let input_zero = (d.get_screen_width() - 200, d.get_screen_height() - 200);
    d.draw_rectangle(input_zero.0, input_zero.1 + 150, 50,50, Color::DARKBLUE);
    d.draw_rectangle_lines(input_zero.0, input_zero.1 + 150, 50,50, Color::DEEPPINK);

    d.draw_rectangle(input_zero.0 + 50, input_zero.1 + 150, 50,50, Color::DARKBLUE);
    d.draw_rectangle_lines(input_zero.0 + 50, input_zero.1 + 150, 50,50, Color::DEEPPINK);

    d.draw_rectangle(input_zero.0 + 100, input_zero.1 + 150, 50,50, Color::DARKBLUE);
    d.draw_rectangle_lines(input_zero.0 + 100, input_zero.1 + 150, 50,50, Color::DEEPPINK);

    d.draw_rectangle(input_zero.0 + 50, input_zero.1 + 100, 50,50, Color::DARKBLUE);
    d.draw_rectangle_lines(input_zero.0 + 50, input_zero.1 + 100, 50,50, Color::DEEPPINK);

    d.draw_rectangle(input_zero.0 + 150, input_zero.1 + 100, 50,50, Color::DARKBLUE);
    d.draw_rectangle_lines(input_zero.0 + 150, input_zero.1 + 100, 50,50, Color::DEEPPINK);

    d.draw_rectangle(input_zero.0 + 100, input_zero.1 + 150, 50,50, Color::DARKBLUE);
    d.draw_rectangle_lines(input_zero.0 + 100, input_zero.1 + 150, 50,50, Color::DEEPPINK);

    let binds: &Keybinds = &settings.keybinds;

    let mut color: Color = if d.is_key_down(binds.left){Color::GREEN} else {Color::RED};
    d.draw_text("<", input_zero.0 + 15, input_zero.1 + 150,50, color);

    color = if d.is_key_down(binds.down){Color::GREEN} else {Color::RED};
    d.draw_text("v", input_zero.0 + 50 + 15, input_zero.1 + 150,50, color);

    color = if d.is_key_down(binds.right){Color::GREEN} else {Color::RED};
    d.draw_text(">", input_zero.0 + 100 + 15, input_zero.1 + 150,50, color);

    color = if d.is_key_down(binds.up){Color::GREEN} else {Color::RED};
    d.draw_text("^", input_zero.0 + 50 + 15, input_zero.1 + 100,50, color);

    color = if d.is_key_down(binds.jump){Color::GREEN} else {Color::RED};
    d.draw_text("j", input_zero.0 + 150 + 15, input_zero.1 + 100,50, color);


}


fn to_1_0(b: bool) -> i32 {
    match b {
        true => 1,
        false => 0,
    }
}
