mod messages;
mod challenges;

use crate::messages::output::messages_output_types::MessageOutputType;
use crate::messages::output::message_subscribe::Subscribe;
use crate::messages::output::message_challenge_result::ChallengeResult;
use crate::messages::input::messages_input_types::MessageInputType;
use crate::messages::input::message_subscribe_result::{SubscribeResult, SubscribeError};
use std::{default, env};
use std::str;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};
use std::process::exit;
use byteorder::{ByteOrder, BigEndian};

fn main() {

    let addr = get_addr();
    let stream = TcpStream::connect(addr).expect("Connexion failed");

    let cloned_stream = stream.try_clone().expect("Error cloning stream");
    send(cloned_stream, MessageOutputType::Hello);

    loop {
        let message : MessageInputType = read(stream.try_clone().expect("Error cloning stream"));
        match message {
            MessageInputType::Welcome(welcome) => {
                println!("version : {}", welcome.version);

                let cloned_stream = stream.try_clone().expect("Error cloning stream");
                let subscribe_message = Subscribe{ name: "TEMA LA PATATE".to_string() };
                send(cloned_stream, MessageOutputType::Subscribe(subscribe_message));
            }
            MessageInputType::Challenge(_) => {}
            MessageInputType::SubscribeResult(result) => {
                match result {
                    SubscribeResult::Ok => {
                        println!("Successfully registered");
                    }
                    SubscribeResult::Err(error) => {
                        match error {
                            SubscribeError::AlreadyRegistered => println!("Error during registration : AlreadyRegistered"),
                            SubscribeError::InvalidName => println!("Error during registration : InvalidName")
                        }
                        exit(409);
                    }
                }
            }
            MessageInputType::ChallengeTimeout(_) => {}
            MessageInputType::PublicLeaderBoardMessage(_) => {}
            MessageInputType::EndOfGame(_) => break,
            MessageInputType::RoundSummary(_) => {}
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
        let nb = BigEndian::read_u32(&nb);

        if nb > 0 {
            let mut str_bytes = vec![0; nb as usize];
            stream.read_exact(&mut str_bytes).expect("Error Reading");
            let str = str::from_utf8(&str_bytes).unwrap();

            let message: MessageInputType = match serde_json::from_str(str) {
                Ok(num) => num,
                Err(_) => continue,
            };
            return message;
        }
    }
}

fn send(mut stream: TcpStream, message: MessageOutputType){

    let str = &*serde_json::to_string(&message).unwrap();
    let str_bytes = str.as_bytes();

    let nb: u32 = str_bytes.len() as u32;

    let mut buf= vec![0; 4];
    BigEndian::write_u32(&mut buf, nb);

    for x in str_bytes {
        buf.push(*x);
    }

    stream.write(&buf).expect("Error Sending Message");
}
