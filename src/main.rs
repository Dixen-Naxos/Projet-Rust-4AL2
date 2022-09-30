use std::env;
use std::str;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};
use byteorder::{ByteOrder, BigEndian};
use serde_json::{json, Value};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Response {
    Welcome: Welcome
}

#[derive(Serialize, Deserialize)]
struct Welcome {
    version: u8,
}

enum SubscribeResult {
    Ok,
    Err(SubscribeError)
}

enum SubscribeError {
    AlreadyRegistered,
    InvalidName
}

fn main() {
    let mut args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return;
    }

    args[1].push_str(":7878");
    let addr = args[1].clone();

    println!("{}\n", addr);

    let stream = TcpStream::connect(addr).expect("Connexion failed");

    println!("1 :");
    let cloned_stream = stream.try_clone().expect("Error cloning stream");
    let str = r#""Hello""#;
    send(cloned_stream, str);

    println!("2 :");
    let cloned_stream = stream.try_clone().expect("Error cloning stream");
    let str = r#"{"Subscribe":{"name":"free_patato"}}"#;
    send(cloned_stream, str);

    stream.shutdown(Shutdown::Both).expect("Error shutdown connexion");
}


fn send(mut stream: TcpStream, str: &str) {

    let str = str.as_bytes();

    let nb: u32 = str.len() as u32;

    let mut buf= vec![0; 4];
    BigEndian::write_u32(&mut buf, nb);

    for x in str {
        buf.push(*x);
    }

    stream.write(&buf).expect("Error Sending Message");

    let mut nb = [0;4];
    stream.read(&mut nb).expect("Error Reading");
    let nb = BigEndian::read_u32(&nb);

    let mut str = vec![0; nb as usize];
    stream.read(&mut str).expect("Error Reading");
    let str = str::from_utf8(&str).unwrap();
    let str: Value = serde_json::from_str(str).expect("Error parsing json");

    println!("version : {}", str["Welcome"]);
}
