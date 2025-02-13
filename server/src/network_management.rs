use crate::file_loader::load_level;
use crate::game_resources::Rectangle;
use crate::player_simple::PlayerSimple;
use itertools::Itertools;
use raylib::math::Vector2;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Instant;

const LEVELTOTRANSMIT: &str = include_str!("level1ForNetworking.txt");

pub(crate) fn start() {
    let level_data = load_level(1);
    let mut players: Vec<PlayerSimple> = Vec::with_capacity(2);
    let listener = TcpListener::bind("127.0.0.1:9998").expect("Could not bind on port 8080");
    println!("Listening on {}", listener.local_addr().unwrap());
    let mut prev_update_time = Instant::now();
    for stream in listener.incoming() {
        update_players(
            &mut players,
            &level_data,
            prev_update_time.elapsed().as_nanos() - Instant::now().elapsed().as_nanos(),
        );
        println!("update time {}", prev_update_time.elapsed().as_nanos());
        prev_update_time = Instant::now();
        match stream {
            Ok(stream) => {
                handle_client(stream, &mut players);
            }
            Err(e) => {
                eprintln!("Failed to establish connection: {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream, players: &mut Vec<PlayerSimple>) {
    let mut buffer = [0; 256];
    stream
        .read(&mut buffer)
        .expect("Could not read from stream");
    let mut request = String::from_utf8_lossy(&buffer[..]).to_string();
    request = request.trim_end_matches(char::from(0)).to_string();
    let breakout_test = request.chars().collect_vec();
    if breakout_test[0] == '!' {
        println!("Got !!");
        players.remove(breakout_test[1].to_digit(10).unwrap() as usize - 1);
        return;
    }
    let request_vec = request
        .chars()
        .map(|x| x.to_digit(10).unwrap() as i32)
        .collect::<Vec<i32>>();
    let inputs = request_vec.split_at(4);
    let mut player_number: usize = inputs.1[0] as usize;

    if player_number == 0 {
        player_number = (players.len() + 1) as usize;
        players.push(PlayerSimple::new(Vector2::new(500.0, 500.0)));
    }
    if player_number > 0 {
        players[player_number - 1].set_inputs(inputs.0);
    }

    let mut final_output: String = player_number.to_string() + "|";
    final_output = final_output.to_string() + LEVELTOTRANSMIT;
    for i in 0..players.len() {
        let ph = &players[i].hitbox.hitbox;
        final_output += &*("\"".to_owned() + &*(i + 7).to_string() + "\" : {\n");
        final_output += &*("    \"x\": ".to_owned() + ph.x.to_string().as_str() + ",\n");
        final_output += &*("    \"y\": ".to_owned() + ph.y.to_string().as_str() + ",\n");
        final_output += &*("    \"w\": ".to_owned() + ph.width.to_string().as_str() + ",\n");
        final_output += &*("    \"h\": ".to_owned() + ph.height.to_string().as_str() + ",\n");
        final_output += "    \"c\": {\n";
        final_output += "        \"r\": 255,\n";
        final_output += "        \"g\": 100,\n";
        final_output += "        \"b\": 100,\n";
        final_output += "        \"a\": 255\n";
        final_output += "       }\n";
        final_output += "   },\n";
    }
    final_output += &*("\"length\": ".to_owned() + &*(6 + players.len()).to_string());
    final_output += "}\n";

    let response = final_output.as_bytes();

    stream.write(response).expect("Could not write to stream");
}

fn update_players(players: &mut Vec<PlayerSimple>, level: &Vec<Rectangle>, dt: u128) {
    for i in 0..players.len() {
        let mut level_clone = level.clone();
        for j in 0..i {
            level_clone.push(players[j].hitbox.hitbox.clone());
        }
        for k in i + 1..players.len() {
            level_clone.push(players[k].hitbox.hitbox.clone());
        }
        players[i].update(&level_clone, dt);
    }
}
