use std::env;
use std::str;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};
use std::ptr::null;
use byteorder::{ByteOrder, BigEndian};
use serde_json::{json, Value};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ChallengeResult {
    ChallengeResult : ChallengeResultValue
}

struct ChallengeResultValue {
    answer : ChallengeAnswer,
    next_target : String
}

struct ChallengeAnswer {
    answer:String,
    seed:u8,
    hashcode:String
}

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
    let welcome : Value = send(cloned_stream, str);
    println!("version : {}",welcome["Welcome"]["version"]);



    println!("2 :");
    let cloned_stream = stream.try_clone().expect("Error cloning stream");
    let str = r#"{"Subscribe":{"name":"free_patato"}}"#;
    let subscribeResult : Value = send(cloned_stream, str);
    let res = subscribeResult["SubscribeResult"]["Err"].to_string();
    if res != "null" {
        println!("SubscribeResult : {}", subscribeResult["SubscribeResult"]["Err"]);
    }else{
        println!("SubscribeResult : {}", subscribeResult["SubscribeResult"]);
    }

    while true {
        //stream.try_clone().expect("Error cloning stream").read(&mut nb).expect("Error Reading");
        let mut nb = [0;4];
        stream.try_clone().expect("Error cloning stream").read(&mut nb).expect("Error Reading");
        let nb = BigEndian::read_u32(&nb);

        let mut str = vec![0; nb as usize];
        stream.try_clone().expect("Error cloning stream").read(&mut str).expect("Error Reading");
        let str = str::from_utf8(&str).unwrap();
        let str: Value = serde_json::from_str(str).expect("Error parsing json");
        //println!("response : {}",str);
        if nb > 0 {
            println!("message : {}", str);
            if(str["Challenge"].to_string() != "null"){

                let mut message = str["Challenge"]["MD5HashCash"]["message"].to_string();
                let message2 = message[1..message.len() - 1].to_string();
                let mut find = 0;
                let mut seed = 0;
                while find == 0 {
                    let val = md5::compute(hex::encode( seed.to_string() ) + &message2);
                    let binary_value = convert_to_binary_from_hex(val);
                    for i in 0..str["Challenge"]["MD5HashCash"]["complexity"].parse {
                        if binary_value[i] == '0' {
                            find = 1
                        }else {
                            find = 0
                        }
                    }
                    if find == 0{
                        seed = seed+1;
                    }
                }
                //ChallengeResultHere
            }
        }
    }

    stream.shutdown(Shutdown::Both).expect("Error shutdown connexion");
}

fn convert_to_binary_from_hex(hex: &str) -> String {
    hex[2..].chars().map(to_binary).collect()
}

fn to_binary(c: char) -> &'static str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => "",
    }
}


fn send(mut stream: TcpStream, str: &str) -> Value {

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

    str
}
