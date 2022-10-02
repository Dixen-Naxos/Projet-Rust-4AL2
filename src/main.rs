use std::{default, env};
use std::str;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};
use std::ptr::null;
use byteorder::{ByteOrder, BigEndian};
use hex::FromHexError;
use serde_json::{json, Value};
use serde::{Deserialize, Serialize};
use hexutil;

#[derive(Serialize, Deserialize)]
struct ChallengeResult {
    ChallengeResult : ChallengeResultValue
}
#[derive(Serialize, Deserialize)]
struct ChallengeResultValue {
    answer : MD5HashCash,
    next_target : String
}

#[derive(Serialize, Deserialize)]
struct MD5HashCash {
    MD5HashCash : MD5HashCashValue
}

#[derive(Serialize, Deserialize)]
struct MD5HashCashValue {
    seed : u64,
    hashcode : String
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
    send(cloned_stream, str);
    let welcome : Value = read(stream.try_clone().expect("Error cloning stream"));
    println!("version : {}",welcome["Welcome"]["version"]);



    println!("2 :");
    let cloned_stream = stream.try_clone().expect("Error cloning stream");
    let str = r#"{"Subscribe":{"name":"free_patato"}}"#;
    send(cloned_stream, str);
    let subscribeResult : Value = read(stream.try_clone().expect("Error cloning stream"));
    let res = subscribeResult["SubscribeResult"]["Err"].to_string();
    if res != "null" {
        println!("SubscribeResult : {}", subscribeResult["SubscribeResult"]["Err"]);
    }else{
        println!("SubscribeResult : {}", subscribeResult["SubscribeResult"]);
    }

    while true {
        let str = read(stream.try_clone().expect("Error cloning stream"));
        //println!("message : {}", str);
        if str["Challenge"].to_string() != "null" {
            let mut message = str["Challenge"]["MD5HashCash"]["message"].to_string();
            //let mut message = "The Isa's funny basket eats our nervous basket.".to_string();

            let message = message[1..message.len() - 1].to_string();
            let mut find = 0;
            let mut seed = 0;
            //let mut binary_value= "".to_string();
            let mut completeSeed = "".to_owned();
            while completeSeed.len() < 16-hex::encode( seed.to_string()).len() {
                completeSeed.push('0');
            }
            completeSeed.push_str(&*hex::encode(seed.to_string()));
            let mut val = md5::compute(completeSeed.clone() + &message); // a modifier
            let momo = str["Challenge"]["MD5HashCash"]["complexity"].to_string().parse::<i32>().unwrap();
            //let momo = "16".to_string().parse::<i32>().unwrap();
            while true {
                completeSeed = "".to_string();
                while completeSeed.len() < 16-hex::encode( seed.to_string()).len() {
                    completeSeed.push('0');
                }
                completeSeed.push_str(&*hex::encode(seed.to_string()));
                val = md5::compute(completeSeed.clone() + &message);
                let mut binary_value = convert_to_binary_from_hex( &*format!("{:X}", val) ).to_string();
                binary_value = binary_value[0..momo as usize].to_string();
                //println!("binary : {}", binary_value);
                if isize::from_str_radix(&*binary_value, 2).unwrap() == 0 {
                    break
                }
                /*for i in 0..momo {
                    if binary_value.chars().nth(i as usize).unwrap() == '0' {
                        find = find+1
                    }
                }*/
                //if find < momo{
                seed = seed+1;
                //find = 0;
               // }
            }

            let MD5HashCashValue : MD5HashCashValue = MD5HashCashValue {
                //seed : "0x".to_string()+ &*completeSeed,
                seed : u64::from_str_radix(&*completeSeed, 16).expect("Ta race"),
                hashcode : format!("{:X}", val)
            };

            let MD5HashCash : MD5HashCash = MD5HashCash {
                MD5HashCash : MD5HashCashValue
            };

            let ChallengeResultValue : ChallengeResultValue = ChallengeResultValue {
                answer : MD5HashCash,
                next_target : "free_patato".to_string()
            };

            let result : ChallengeResult = ChallengeResult{
                ChallengeResult : ChallengeResultValue
            };
            //println!("seed : {}", result.ChallengeResult.answer.MD5HashCash.seed);
            let mut json : String = serde_json::to_string(&result).unwrap();
            //let json = json[0..52].to_string() + &*json[53..71].to_string() + &*json[72..json.len() - 1].to_string();

            //let message = message[1..message.len() - 1].to_string();
            //println!("result : {}", json);
            //println!("val : {:X} and seed : {} and binaryvalue : {} and message : {}", val, seed,binary_value,message);
            //println!("3 :");
            let cloned_stream = stream.try_clone().expect("Error cloning stream");
            let str = json;
            send(cloned_stream, &*str);
        }
    }

    stream.shutdown(Shutdown::Both).expect("Error shutdown connexion");
}

pub fn hex_to_u64(b: &[u8]) -> Option<u64> {
    let a = std::str::from_utf8(b).ok()?;
    u64::from_str_radix(a, 16).ok()
}

fn convert_to_binary_from_hex(hex: &str) -> String {
    hex.chars().map(to_binary).collect()
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


fn send(mut stream: TcpStream, str: &str){

    let str = str.as_bytes();

    let nb: u32 = str.len() as u32;

    let mut buf= vec![0; 4];
    BigEndian::write_u32(&mut buf, nb);

    for x in str {
        buf.push(*x);
    }

    stream.write(&buf).expect("Error Sending Message");


}

fn read (mut stream: TcpStream) -> Value {
    let str : Value = Default::default();
    while true {
        let mut nb = [0;4];
        stream.read(&mut nb).expect("Error Reading");
        let nb = BigEndian::read_u32(&nb);

        if nb > 0 {
            let mut str = vec![0; nb as usize];
            stream.read(&mut str).expect("Error Reading");
            let str = str::from_utf8(&str).unwrap();
            let str: Value = serde_json::from_str(str).expect("Error parsing json");
            return str;
        }

    }

    str
}
