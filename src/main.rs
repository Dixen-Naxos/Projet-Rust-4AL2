mod messages;
mod challenges;

use std::{default, env};
use std::str;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};
use std::ptr::null;
use std::time::Instant;
use byteorder::{ByteOrder, BigEndian};
use hex::FromHexError;
use serde_json::{json, Value};
use serde::{Deserialize, Serialize};
use hexutil;
use std::sync::mpsc;
use std::thread;

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
    // let welcome : Welcome = serde_json::from_str(&*read(stream.try_clone().expect("Error cloning stream")).to_string()).unwrap();
    // println!("version : {}", welcome.welcome.version);
}

fn read (mut stream: TcpStream) -> Value {
    let str : Value = Default::default();
    while true {
        let mut nb = [0;4];
        stream.read(&mut nb).expect("Error Reading");
        let nb = BigEndian::read_u32(&nb);

        if nb > 0 {
            let mut str = vec![0; nb as usize];
            stream.read_exact(&mut str).expect("Error Reading");
            let str2 = str::from_utf8(&str).unwrap();

            let str: Value = match serde_json::from_str(str2) {
                Ok(num) => num,
                Err(_) => continue,
            };
            return str;
        }

    }

    str
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

/*
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
    let welcome : Welcome = serde_json::from_str(&*read(stream.try_clone().expect("Error cloning stream")).to_string()).unwrap();
    println!("version : {}", welcome.welcome.version);

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

        // .\server.exe --debug -g nonogram-solver
        //nonogram_solver(stream.try_clone().expect("Error cloning stream"));

        // .\server.exe --debug -g recover-secret
        recover_secret_solver(stream.try_clone().expect("Error cloning stream"));

        //MD5_solver(stream.try_clone().expect("Error cloning stream"));

        //MD5(stream.try_clone().expect("Error cloning stream"));
    }

    stream.shutdown(Shutdown::Both).expect("Error shutdown connexion");
}

fn recover_secret_solver(mut stream: TcpStream) {
    let str = read(stream.try_clone().expect("Error cloning stream"));

    if str["Challenge"].to_string() != "null" {

        let _data : RecoverSecretInput = serde_json::from_str(&*str["Challenge"]["RecoverSecret"].to_string()).unwrap();

        let _test = RecoverSecret::new(_data);

        let _out : String = _test.solve();

        //println!("out : {}", _out);

        let solver_out = RecoverSecretOutput {
            secret_sentence: _out
        };

        let answer : RecoverSecretAnswer = RecoverSecretAnswer {
            RecoverSecret : solver_out
        };

        let result_value: RecoverSecretValue = RecoverSecretValue {
            answer : answer,
            next_target : "free_patato".to_string()
        };

        let result : RecoverSecretResult = RecoverSecretResult{
            ChallengeResult : result_value
        };

        send(stream.try_clone().expect("Error cloning stream"), &*serde_json::to_string(&result).unwrap());
    }
}

fn nonogram_solver(mut stream: TcpStream) {
    let str = read(stream.try_clone().expect("Error cloning stream"));

    if str["Challenge"].to_string() != "null" {

        let _data : NonogramSolverInput = serde_json::from_str(&*str["Challenge"]["NonogramSolver"].to_string()).unwrap();

        let _test = Nonogram::new(_data);

        let _out : Vec<Vec<bool>> = Nonogram::solve(&_test);

        let solver_out = NonogramSolverOutput {
            grid: Nonogram::_vec_to_string(_out)
        };

        let answer : NonogramAnswer = NonogramAnswer {
            NonogramSolver : solver_out
        };

        let result_value: NonogramResultValue = NonogramResultValue {
            answer : answer,
            next_target : "free_patato".to_string()
        };

        let result : NonogramResult = NonogramResult{
            ChallengeResult : result_value
        };

        send(stream.try_clone().expect("Error cloning stream"), &*serde_json::to_string(&result).unwrap());
    }
}

fn MD5_solver(mut stream: TcpStream) {

    let str = read(stream.try_clone().expect("Error cloning stream"));
    //println!("message : {}", str);
    if str["Challenge"].to_string() != "null" {

        //println!("seed : {}", result.ChallengeResult.answer.MD5HashCash.seed);
        let _data : Md5HashCashInput = Md5HashCashInput {
            message : str["Challenge"]["MD5HashCash"]["message"].to_string(),
            complexity : str["Challenge"]["MD5HashCash"]["complexity"].to_string().parse::<i32>().unwrap()
        };

        let _md5 = Md5::new(_data);

        let MD5HashCash : MD5HashCash = MD5HashCash {
            MD5HashCash : _md5.solve() // mds::solve
        };

        let ChallengeResultValue : ChallengeResultValue = ChallengeResultValue {
            answer : MD5HashCash,
            next_target : "free_patato".to_string()
        };
        let result : ChallengeResult = ChallengeResult{
            ChallengeResult : ChallengeResultValue
        };
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
            stream.read_exact(&mut str).expect("Error Reading");
            let str2 = str::from_utf8(&str).unwrap();

            let str: Value = match serde_json::from_str(str2) {
                Ok(num) => num,
                Err(_) => continue,
            };
            return str;
        }

    }

    str
}

*/
