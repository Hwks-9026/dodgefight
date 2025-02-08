extern crate core;

use crate::file_loader::load_level;
use crate::game_resources::Rectangle;
use crate::settings::{Keybinds, Settings};
use raylib::prelude::*;
use std::io::{Read, Write};
use std::net::TcpStream;

pub(crate) fn game_loop(mut rl: RaylibHandle, thread: RaylibThread, settings: Settings, address: String) {
    let binds = &settings.keybinds;
    let mut player_number: u8 = 0;
    if settings.fullscreen {
        rl.toggle_fullscreen()
    }
    while !rl.window_should_close() {
        let mut packet: String = "".to_string();
        packet += &*to_1_0(rl.is_key_down(binds.left)).to_string();
        packet += &*to_1_0(rl.is_key_down(binds.right)).to_string();
        packet += &*to_1_0(rl.is_key_pressed(binds.jump)).to_string();
        packet += &*to_1_0(rl.is_key_down(binds.jump)).to_string();
        packet += &*(player_number).to_string();
        let mut stream: TcpStream =
            TcpStream::connect(address.clone()).expect("Could not connect to server");
        _ = stream.write(packet.as_bytes());
        _ = stream.flush().expect("Could not flush stream");
        let mut read_buffer = [0; 4096];

        stream
            .read(&mut read_buffer)
            .expect("Could not read from server");
        let mut result: &str = &*String::from_utf8_lossy(&*Vec::from(&read_buffer[..])).to_string();
        result = result.trim_matches(char::from(0));
        let mut segment: usize = 0;
        for result_element in result.split("|") {
            match segment {
                0 => {
                    if player_number == 0 {
                        player_number = result_element.parse().unwrap();
                    }
                }
                1 => draw_frame(&mut rl, &thread, &settings, &load_level(result_element)),
                _ => {}
            }
            segment += 1;
        }
    }
    terminate_connection(player_number);
}
fn terminate_connection(player_number: u8) {
    let mut stream = TcpStream::connect("127.0.0.1:9999").expect("Could not connect to server");
    let packet: String = "!".to_string() + &player_number.to_string();
    _ = stream.write(packet.as_bytes());
    _ = stream.flush().expect("Could not flush stream");
}
fn draw_frame(rl: &mut RaylibHandle, thread: &RaylibThread, settings: &Settings, level_data: &Vec<Rectangle>) {
    let mut d: RaylibDrawHandle = rl.begin_drawing(&thread);
    d.clear_background(Color::BLACK);

    for r in level_data {
        d.draw_rectangle(
            r.x as i32,
            r.y as i32,
            r.width as i32,
            r.height as i32,
            r.color,
        );
    }

    if settings.debug {
        draw_debug_hud(&mut d, settings);
    }
}

fn draw_debug_hud(d: &mut RaylibDrawHandle, settings: &Settings) {
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

    let binds: &Keybinds = &settings.keybinds;

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

fn to_1_0(b: bool) -> i32 {
    match b {
        true => 1,
        false => 0,
    }
}
