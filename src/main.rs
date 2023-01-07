mod messages;
mod challenges_compute;

use crate::messages::output::messages_output_types::MessageOutputType;
use crate::messages::output::message_subscribe::Subscribe;
use crate::messages::output::message_challenge_result::ChallengeResult;
use crate::messages::input::messages_input_types::MessageInputType;
use std::env;
use std::str;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};
use std::process::exit;

fn main() {

    let addr = get_addr();
    let stream = TcpStream::connect(addr).expect("Connexion failed");

    let cloned_stream = stream.try_clone().expect("Error cloning stream");
    send(cloned_stream, MessageOutputType::Hello);

    loop {
        let message : MessageInputType = read(stream.try_clone().expect("Error cloning stream"));
        let message_out = message.match_type();
        match message_out {
            Some(message) => {
                let cloned_stream = stream.try_clone().expect("Error cloning stream");
                send(cloned_stream, message);
            },
            None => {}
        }
    }

    stream.shutdown(Shutdown::Both).expect("Error shutdown connexion");
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

fn read (mut stream: TcpStream) -> MessageInputType {
    loop {
        let mut nb = [0; 4];
        stream.read(&mut nb).expect("Error Reading");
        let nb = i32::from_be_bytes(nb);

        if nb > 0 {
            let mut str_bytes = vec![0; nb as usize];
            stream.read_exact(&mut str_bytes).expect("Error Reading");
            let str = str::from_utf8(&str_bytes).unwrap();
            println!("Read : {}", str);

            let message: MessageInputType = match serde_json::from_str(str) {
                Ok(message) => message,
                Err(_) => continue
            };
            return message;
        }
    }
}

fn send(mut stream: TcpStream, message: MessageOutputType){

    let str = &*serde_json::to_string(&message).unwrap();
    println!("Send : {}", str);
    let str_bytes = str.as_bytes();

    let nb: u32 = str_bytes.len() as u32;

    let mut buf= vec![0; 4];
    buf = Vec::from(nb.to_be_bytes());

    for x in str_bytes {
        buf.push(*x);
    }

    stream.write(&buf).expect("Error Sending Message");
}
