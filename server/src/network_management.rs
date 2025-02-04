use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use raylib::math::Vector2;
use crate::file_loader::load_level;
use crate::game_resources::Rectangle;
use crate::player_simple;
use crate::player_simple::PlayerSimple;

const LEVEL: &str = include_str!("level1.json");
const LEVELTOTRANSMIT: &str = include_str!("level1ForNetworking.txt");

pub(crate) fn start() {
    let level_data = load_level(1);
    let mut player = player_simple::PlayerSimple::new(Vector2::new(500.0, 500.0), 1);
    let listener = TcpListener::bind("127.0.0.1:9999").expect("Could not bind on port 8080");
    println!("Listening on {}", listener.local_addr().unwrap());
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream, &mut player, &level_data);
            }
            Err(e) => { eprintln!("Failed to establish connection: {}", e); }
        }
    }
}

fn handle_client(mut stream: TcpStream, player: &mut PlayerSimple, level_data: &Vec<Rectangle>) {
    let mut buffer = [0; 256];
    stream.read(&mut buffer).expect("Could not read from stream");
    let mut request = String::from_utf8_lossy(&buffer[..]).to_string();
    request = (request.trim_end_matches(char::from(0))).to_string();

    let inputs = request.chars().map(|x| x.to_digit(10).unwrap() as i32).collect::<Vec<i32>>();
    player.update(inputs, level_data);
    let mut final_output = LEVELTOTRANSMIT.to_string();
    let ph = &player.hitbox.hitbox;
    final_output += "\"7\" : {\n";
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
    final_output += "   }\n";
    final_output += "}\n";


    let response = final_output.as_bytes();
    stream.write(response).expect("Could not write to stream");
}
