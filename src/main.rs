mod messages;
mod challenges_compute;

use crate::messages::output::messages_output_types::MessageOutputType;
use crate::messages::output::message_subscribe::Subscribe;
use crate::messages::output::message_challenge_result::ChallengeResult;
use crate::messages::input::messages_input_types::{MessageInputResult, MessageInputType};
use std::{env, thread};
use std::str;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};
use std::process::exit;
use std::str::Utf8Error;
use std::sync::mpsc;

fn main() {

    let sessions = Vec::from(["TEMA LA PATATE".to_string(), "Rust".to_string()]);

    let (tx, rx) = mpsc::channel();

    for session in sessions {
        let tx1 = tx.clone();
        thread::spawn(move || {
            start_game(session.clone());
            tx1.send(session);
        });
    }

    for received in rx {
        println!("{} is dead", received);
    }

    //start_game("TEMA LA PATATE".to_string());
}

fn start_game(self_name: String) {
    let addr = get_addr();
    let stream = match TcpStream::connect(addr) {
        Ok(tcpStream) => tcpStream,
        Err(_) => exit(404)
    };

    send(&stream, MessageOutputType::Hello);

    let mut player_to_kill = self_name.clone();

    loop {
        let message : MessageInputType = read(&stream);
        let message_out = message.match_type(player_to_kill.clone(), &self_name);
        match message_out {
            MessageInputResult::MessageOutputType(message) => {
                send(&stream, message);
            },
            MessageInputResult::PlayerToKill(player) => player_to_kill = player,
            MessageInputResult::Exit => break,
            MessageInputResult::None => {}
        }
    }

    stream.shutdown(Shutdown::Both);
}

fn get_addr() -> String {

    let mut args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        exit(300);
    }

    args[1].push_str(":7878");
    let addr = args[1].clone();

    addr
}

fn read (mut stream: &TcpStream) -> MessageInputType {
    loop {
        let mut nb = [0; 4];
        stream.read(&mut nb);
        let nb = i32::from_be_bytes(nb);

        if nb > 0 {
            let mut str_bytes = vec![0; nb as usize];
            stream.read_exact(&mut str_bytes);
            let str = match str::from_utf8(&str_bytes) {
                Ok(str) => str,
                Err(_) => ""
            };
            println!("Read : {}", str);

            let message: MessageInputType = match serde_json::from_str(str) {
                Ok(message) => message,
                Err(_) => continue
            };
            return message;
        }
    }
}

fn send(mut stream: &TcpStream, message: MessageOutputType){

    let str = match serde_json::to_string(&message) {
        Ok(str) => str,
        Err(_) => "".to_string()
    };
    println!("Send : {}", str);
    let str_bytes = str.as_bytes();

    let nb: u32 = str_bytes.len() as u32;

    let mut buf= vec![0; 4];
    buf = Vec::from(nb.to_be_bytes());

    for x in str_bytes {
        buf.push(*x);
    }

    stream.write(&buf);
}
