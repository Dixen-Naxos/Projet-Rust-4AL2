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

#[derive(Serialize)]
struct NonogramResult {
    ChallengeResult : NonogramResultValue
}

#[derive(Serialize)]
struct NonogramResultValue {
    answer : NonogramAnswer,
    next_target : String
}

#[derive(Serialize)]
struct NonogramAnswer {
    NonogramSolver : NonogramSolverOutput
}

#[derive(Deserialize)]
pub struct NonogramSolverInput {
    pub rows: Vec<Vec<u32>>,
    pub cols: Vec<Vec<u32>>,
}

#[derive(Serialize)]
pub struct NonogramSolverOutput {
    pub grid: String,
}

pub struct Md5HashCashInput {
    pub complexity : i32,
    pub message : String
}

trait Challenge {

    type Input;
    type Output;

    fn name() -> String;

    fn new(input: Self::Input) -> Self;

    fn solve(&self) -> Self::Output;

    fn verify(&self, answer: &Self::Output) -> bool;
}

struct Nonogram {
    input: NonogramSolverInput
}

struct Md5 {
    input : Md5HashCashInput
}

impl Nonogram {

    fn create_solution(line: &Vec<Vec<u32>>, nb: usize) -> Vec<Vec<Vec<bool>>> {

        let mut r: Vec<Vec<Vec<bool>>> = Vec::new();

        for i in line {
            let groups = i.len();
            let mut sum = 0;
            let mut nb_empty = 0;
            for j in i {
                sum += *j;
            }
            if groups > 0 {
                nb_empty = 1 + nb - sum as usize - groups;
            } else {
                nb_empty = nb;
            }
            r.push(Nonogram::_create_solution(nb_empty, groups, nb, i));
        }

        r
    }

    fn _create_solution(nb_empty: usize, groups: usize, nb: usize, line: &Vec<u32>) -> Vec<Vec<bool>> {

        let mut r: Vec<Vec<bool>> = Vec::new();
        let mut combi: Vec<Vec<bool>> = Nonogram::_create_combi(nb_empty + groups, groups);
        let mut v: Vec<bool>;
        let mut cnt: usize;
        let mut index: usize;

        for p in combi {
            v = Vec::new();
            cnt = 0;
            index = 0;

            for i in 0..p.len() {
                if p[i] {
                    for j in 0..(*line)[index] {
                        v.push(true);
                    }
                    if index < (*line).len() - 1 {
                        v.push(false);
                    }
                    index += 1;
                } else {
                    v.push(false);
                }
            }

            r.push(v);
        }

        r
    }

    fn _create_combi(range: usize, nb: usize) -> Vec<Vec<bool>> {

        let mut r: Vec<Vec<bool>> = Vec::new();
        let mut a_range: Vec<bool> = Vec::new();

        for i in 0..range {
            a_range.push(false);
        }

        r = Nonogram::_combi(a_range, nb, 0, 0, r.clone());

        r
    }

    fn _combi(mut array: Vec<bool>, nb: usize, index: usize, cnt: usize, mut r: Vec<Vec<bool>>) -> Vec<Vec<bool>> {

        if cnt == nb {
            r.push(array);
            return r;
        }

        if index - cnt > array.len() - nb {
            return r;
        }

        let r = Nonogram::_combi(array.clone(), nb, index + 1, cnt, r.clone());

        array[index] = true;
        let r = Nonogram::_combi(array.clone(), nb, index + 1, cnt + 1, r.clone());

        r
    }

    fn _solve(mut s: Vec<u32>, cols: Vec<Vec<Vec<bool>>>, rows: Vec<Vec<u32>>, index: usize) -> Vec<Vec<bool>> {

        let mut grid: Vec<Vec<bool>> = Vec::new();

        for i in 0..cols.len() {
            let a = s[i] as usize;
            grid.push(cols[i][a].clone());
        }

        let verif = Nonogram::_verify(grid.clone(), rows.clone());

        if verif == -1 {
            return grid;
        }

        if index == cols.len() || verif < index as isize {
            return Vec::new();
        }

        let (tx, rx) = mpsc::channel();

        for i in 0..cols[index].len() {
            let tx1 = tx.clone();
            let mut grid_t = grid.clone();
            let mut s_t = s.clone();
            let cols_t = cols.clone();
            let rows_t = rows.clone();
            thread::spawn(move || {
                s_t[index] = i as u32;
                grid_t = Nonogram::_solve(s_t.clone(), cols_t.clone(), rows_t.clone(), index + 1);
                tx1.send(grid_t).unwrap();
            });
        }

        for received in rx {
            if received.len() != 0 {
                return received;
            }
        }

        Vec::new()
    }

    fn _verify(grid: Vec<Vec<bool>>, rows: Vec<Vec<u32>>) -> isize {

        let mut index: usize;
        let mut cnt: u32;

        for j in 0..rows.len() {
            index = 0;
            cnt = 0;

            for i in 0..grid.len() {
                if grid[i][j] {
                    if index == rows[j].len() {
                        return i as isize;
                    }
                    cnt += 1;
                    continue;
                }
                if grid[i][j] == false && cnt != 0 {
                    if cnt != rows[j][index] {
                        return i as isize;
                    }
                    cnt = 0;
                    index += 1;
                }
            }
            if cnt != 0 && cnt != rows[j][index] {
                return grid.len() as isize;
            }
        }
        -1
    }

    fn _solve_rows(s: &mut Vec<u32>, rows: &Vec<Vec<Vec<bool>>>, cols: &Vec<Vec<u32>>, index: usize) -> Vec<Vec<bool>> {

        let verify = Nonogram::_verify_rows(s, rows, cols);
        let mut grid = Vec::new();

        if verify == -1 {
            let mut v;
            for i in 0..(*cols).len() {
                v = Vec::new();
                for j in 0..(*rows).len() {
                    v.push((*rows)[j][(*s)[j] as usize][i]);
                }
                grid.push(v);
            }
            return grid;
        }

        if index == (*rows).len() || verify < index as isize {
            return grid;
        }

        for i in 0..(*rows)[index].len() {
            (*s)[index] = i as u32;
            grid = Nonogram::_solve_rows(s, rows, cols, index + 1);
            if grid.len() != 0 {
                return grid;
            }
        }

        grid
    }

    fn _solve_rows_thearded(mut s: Vec<u32>, rows: &Vec<Vec<Vec<bool>>>, cols: &Vec<Vec<u32>>, index: usize) -> Vec<Vec<bool>> {

        let verify = Nonogram::_verify_rows(&s, rows, cols);
        let mut grid = Vec::new();

        if verify == -1 {
            let mut v;
            for i in 0..(*cols).len() {
                v = Vec::new();
                for j in 0..(*rows).len() {
                    v.push((*rows)[j][s[j] as usize][i]);
                }
                grid.push(v);
            }

            return grid;
        }

        if index == (*rows).len() || verify < index as isize {
            return grid;
        }

        let (tx, rx) = mpsc::channel();

        for i in 0..(*rows)[index].len() {
            let tx1 = tx.clone();
            let rows_t = rows.clone();
            let cols_t = cols.clone();
            let mut s_t = s.clone();
            thread::spawn(move || {
                s_t[index] = i as u32;
                tx1.send(Nonogram::_solve_rows(&mut s_t, &rows_t, &cols_t, index + 1)).unwrap();
            });
        }

        for _ in 0..rows[index].len() {
            match rx.recv() {
                Ok(data) => {
                    if data.len() != 0 {
                        return data;
                    }
                },
                Err(e) => println!("Une erreur s'est produite : {:?}", e)
            };
        }

        /*for received in rx {
            if received.len() != 0 {
                return received;
            }
        }*/

        grid
    }

    fn _verify_rows(s: &Vec<u32>, rows: &Vec<Vec<Vec<bool>>>, cols: &Vec<Vec<u32>>) -> isize {

        let mut index: usize;
        let mut cnt: u32;

        for i in 0..(*cols).len() {

            index = 0;
            cnt = 0;

            for j in 0..(*rows).len() {
                if (*rows)[j][(*s)[j] as usize][i] {
                    if index == (*cols)[i].len() {
                        return j as isize;
                    }
                    cnt += 1;
                    continue;
                }
                if (*rows)[j][s[j] as usize][i] == false && cnt != 0 {
                    if cnt != (*cols)[i][index] {
                        return j as isize;
                    }
                    cnt = 0;
                    index += 1;
                }
            }

            if cnt != 0 && cnt != (*cols)[i][index] {
                return rows.len() as isize;
            }
        }
        -1
    }

    fn _vec_to_string(grid: Vec<Vec<bool>>) -> String {

        let mut s: String = String::new();

        if grid.len() == 0 {
            return s;
        }

        for j in 0..grid[0].len() {
            for i in 0..grid.len() {
                if grid[i][j] {
                    s.push('#');
                } else {
                    s.push(' ');
                }
            }
            s.push('\n');
        }

        s
    }
}

impl Challenge for Md5 {
    type Input = Md5HashCashInput;
    type Output = MD5HashCashValue;

    fn name() -> String {
        "Md5".to_string()
    }

    fn new(input: Self::Input) -> Self {
        Md5 {input}
    }

    fn solve(&self) -> Self::Output {
        let now = Instant::now();
        //let mut message = str["Challenge"]["MD5HashCash"]["message"].to_string();
        let mut message = &self.input.message;
        //let mut message = "The Isa's funny basket eats our nervous basket.".to_string();

        let message = message[1..message.len() - 1].to_string();
        let mut find = 0;
        let mut seed = 0;
        //let mut binary_value= "".to_string();
        let mut completeSeed = "0000000000000000".to_string();
        /*while completeSeed.len() < 16-hex::encode( seed.to_string()).len() {
            completeSeed.push('0');
        }*/
        let hexa = hex::encode(seed.to_string());
        completeSeed = completeSeed[0..16 - hexa.len()].to_string();
        completeSeed.push_str(&*hexa.to_string());
        let mut val = md5::compute(completeSeed.clone() + &message); // a modifier
        //let momo = str["Challenge"]["MD5HashCash"]["complexity"].to_string().parse::<i32>().unwrap();
        let momo = (&self).input.complexity;
        //let momo = "16".to_string().parse::<i32>().unwrap();

        while true {
            //while find < momo {
            completeSeed = "0000000000000000".to_string();
            /*while completeSeed.len() < 16-hex::encode( seed.to_string()).len() {
                 completeSeed.push('0');
             }*/
            let hexa = hex::encode(seed.to_string());
            completeSeed = completeSeed[0..16 - hexa.len()].to_string();
            completeSeed.push_str(&*hexa.to_string());
            val = md5::compute(completeSeed.clone() + &message);
            let mut binary_value = convert_to_binary_from_hex( &*format!("{:X}", val) ).to_string();
            binary_value = binary_value[0..momo as usize].to_string();
            //println!("binary : {}", binary_value);
            if isize::from_str_radix(&*binary_value, 2).unwrap() == 0 {
                break
            }
            seed = seed+1;
            /*for i in 0..momo {
                if binary_value.chars().nth(i as usize).unwrap() == '0' {
                    find = find+1
                }
            }
            if find < momo{
                seed = seed+1;
                find = 0;
            }*/
            if now.elapsed().as_millis() > 1900 {
                println!("Viré");
                break
            }
        }
        let elapsed_time = now.elapsed();
        println!("Running boucle while took {} ms.", elapsed_time.as_millis());
        let MD5HashCashValue : MD5HashCashValue = MD5HashCashValue {
            //seed : "0x".to_string()+ &*completeSeed,
            seed : u64::from_str_radix(&*completeSeed, 16).expect("Ta race"),
            hashcode : format!("{:X}", val)
        };
        return MD5HashCashValue;
    }

    fn verify(&self, answer: &Self::Output) -> bool {
        todo!()
    }
}

impl Challenge for Nonogram {

    type Input = NonogramSolverInput;
    type Output = Vec<Vec<bool>>;

    fn name () -> String {
        "Nonogram".to_string()
    }

    fn new(input: Self::Input) -> Self {

        Nonogram {
            input
        }
    }

    fn solve(&self) -> Self::Output {

        let s_rows = Nonogram::create_solution(&self.input.rows, self.input.cols.len());

        let mut a: Vec<u32> = Vec::new();

        for i in 0..self.input.rows.len() {
            a.push(0);
        }

        Nonogram::_solve_rows_thearded(a, &s_rows, &self.input.cols, 0)
    }

    fn verify(&self, answer: &Self::Output) -> bool {
        false
    }
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

        // .\server.exe --debug -g nonogram-solver
        nonogram_solver(stream.try_clone().expect("Error cloning stream"));
        //MD5_solver(stream.try_clone().expect("Error cloning stream"));

        //MD5(stream.try_clone().expect("Error cloning stream"));
    }

    stream.shutdown(Shutdown::Both).expect("Error shutdown connexion");
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

/*test {

use std::path::is_separator;
use std::str::Lines;
use digest::generic_array::arr;
use md5;
use hex;

struct Node {
    cost : isize,
    dead_end: bool,
    children : [Option<Box<Node>>;4],
    x : isize,
    y : isize
}
/*impl Copy for Node { }

impl Clone for Node {
        fn clone(&self) -> Node {
        *self
    }
}*/

fn calculChildrenCost<'a>(arrayLab : &'a mut[&'a mut[char]], mut node: &'a mut Node, startLab_x : isize, startLab_y : isize, endLab_x : isize, endLab_y : isize ) -> &'a mut Node {

    if node.x != 0 {
        node.children[0] = Option::from(Box::new(Node {
            cost: abs(node.x - 1 + startLab_x) + abs(node.y + startLab_y) + abs(node.x - 1 + endLab_x) + abs(node.y + endLab_y),
            dead_end: arrayLab[(node.x-1) as usize][node.y as usize] == '#',
            children: [None, None, None, None],
            x: node.x - 1,
            y: node.y
        }))
    }

    if node.x != (arrayLab.len() - 1) as isize {
        node.children[1] = Option::from(Box::new(Node {
            cost: abs(node.x + 1 + startLab_x) + abs(node.y + startLab_y) + abs(node.x + 1 + endLab_x) + abs(node.y + endLab_y),
            dead_end: arrayLab[(node.x+1) as usize][node.y as usize] == '#',
            children: [None, None, None, None],
            x: node.x + 1,
            y: node.y
        }))
    }

    if node.y != 0 {
        node.children[2] = Option::from(Box::new(Node {
            cost: abs(node.x + startLab_x) + abs(node.y - 1 + startLab_y) + abs(node.x + endLab_x) + abs(node.y - 1 + endLab_y),
            dead_end: arrayLab[node.x as usize][(node.y-1) as usize] == '#',
            children: [None, None, None, None],
            x: node.x,
            y: node.y - 1
        }))
    }

    if node.y != arrayLab[0].len() as isize {
        node.children[3] = Option::from(Box::new(Node {
            cost: abs(node.x + startLab_x) + abs(node.y + 1 + startLab_y) + abs(node.x + endLab_x) + abs(node.y + 1 + endLab_y),
            dead_end: arrayLab[node.x as usize][(node.y+1) as usize] == '#',
            children: [None, None, None, None],
            x: node.x,
            y: node.y + 1
        }))
    }

    return node;
}

fn doYouKnowDaWay<'a>(mut node: Node, mut response: String, arrayLab : &'a mut[&'a mut[char]], startLab_x : isize, startLab_y : isize, endLab_x : isize, endLab_y : isize ) -> String {
    //node = calculChildrenCost(arrayLab,node,startLab_x,startLab_y,endLab_x,endLab_y);
    if node.x != 0 {
        node.children[0] = Option::from(Box::new(Node {
            cost: abs(node.x - 1 + startLab_x) + abs(node.y + startLab_y) + abs(node.x - 1 + endLab_x) + abs(node.y + endLab_y),
            dead_end: arrayLab[(node.x-1) as usize][node.y as usize] == '#',
            children: [None, None, None, None],
            x: node.x - 1,
            y: node.y
        }))
    }

    if node.x != (arrayLab.len() - 1) as isize {
        node.children[1] = Option::from(Box::new(Node {
            cost: abs(node.x + 1 + startLab_x) + abs(node.y + startLab_y) + abs(node.x + 1 + endLab_x) + abs(node.y + endLab_y),
            dead_end: arrayLab[(node.x+1) as usize][node.y as usize] == '#',
            children: [None, None, None, None],
            x: node.x + 1,
            y: node.y
        }))
    }

    if node.y != 0 {
        node.children[2] = Option::from(Box::new(Node {
            cost: abs(node.x + startLab_x) + abs(node.y - 1 + startLab_y) + abs(node.x + endLab_x) + abs(node.y - 1 + endLab_y),
            dead_end: arrayLab[node.x as usize][(node.y-1) as usize] == '#',
            children: [None, None, None, None],
            x: node.x,
            y: node.y - 1
        }))
    }

    if node.y != arrayLab[0].len() as isize {
        node.children[3] = Option::from(Box::new(Node {
            cost: abs(node.x + startLab_x) + abs(node.y + 1 + startLab_y) + abs(node.x + endLab_x) + abs(node.y + 1 + endLab_y),
            dead_end: arrayLab[node.x as usize][(node.y+1) as usize] == '#',
            children: [None, None, None, None],
            x: node.x,
            y: node.y + 1
        }))
    }

    let mut min = 2147498847;
    let mut min_index: isize = -1;
    for i in 0..4 {
        if min > node.children[i].as_ref().unwrap().cost && node.children[i].as_ref().unwrap().dead_end == false{
            min = node.children[i].as_ref().unwrap().cost;
            min_index = i as isize;
        }
    }
    match min_index {
        0 => response.push('^'),
        1 => response.push('v'),
        2 => response.push('>'),
        3 => response.push('<'),
        _ => {
            node.dead_end = true;
            return response[0..response.len()-1].to_string();
        }
    }
    if(arrayLab[node.x as usize][node.y as usize] == 'X'){
        return response
    }
    let node : Node = *node.children[min_index as usize].unwrap();
    response = doYouKnowDaWay( node , response, arrayLab, startLab_x, startLab_y, endLab_x, endLab_y);
    return response;
}

fn abs(nb : isize) -> isize {
    if nb > 0 {
        return nb;
    }else {
        return -nb;
    }
}

fn main() {
    let laby : String = "#I###############################\n# #         # # #     # #   #   #\n# # # ####### # ##### # # # # ###\n# # # # # # #   #   #   # # #   #\n# # # # # # # # # ##### # ### ###\n#   # #   # # #   #     #   #   #\n# # # # ### # ### ### ### ##### #\n# # #   #       #       #       #\n# ### # # ### ##### ######### ###\n#   # #   # # #   #       # #   #\n# ####### # ##### # # ##### # # #\n# #               # # # #     # #\n# # ########### ### # # # # # # #\n# #     # #     # # #   # # # # #\n### # ### ####### ### ##### # ###\n# # #   # #   #     #   # # # # #\n# ### # # # # # ##### ### ### # #\n#   # # #   #     #   #   #   # #\n# ##### # ##### ##### # ### ### #\n#       #   #       # # #       #\n# ##### ##### ### ### # ### ### #\n#   #     # #   # # # #       # #\n# # # # ### ##### # ### # ##### #\n# # # #   # # #   #     #     # #\n# ### ##### # # ### # ### ##### #\n# #                 # #     #   #\n### ######### # ### # # ### # ###\n#   # # #     #   # # # # # #   #\n# ### # # ### ##### ##### # #####\n# # #     # #     #       #     #\n# # # ### # ### ### ##### # ### #\n#   # #   #     #     #   # #   X\n#################################".to_string();
    let split : Lines = laby.lines();
    println!("cols : {}", split.clone().nth(0).expect("Not found"));
    let cols: isize = split.clone().nth(0).expect("Not found").len() as isize;
    println!("cols : {}", cols);
    let rows: isize = split.clone().count() as isize;
    println!("rows : {}", rows);
    //let mut arrayLab = vec![[0 as u8 ; cols]; rows as usize];
    let mut arrayLab_raw = vec!['0'; (cols * rows) as usize];

    // Vector of 'width' elements slices
    let mut arrayLab_base: Vec<_> = arrayLab_raw.as_mut_slice().chunks_mut(cols as usize).collect();

    // Final 2d array `&mut [&mut [_]]`
    let arrayLab = arrayLab_base.as_mut_slice();
    arrayLab[0][0] = '2';
    println!("value : {}", arrayLab[0][0]);
    let mut count: usize = 0;
    let mut startLab_x= 0;
    let mut startLab_y= 0;
    let mut endLab_x = 0;
    let mut endLab_y = 0;
    for i in 0..rows {
        for j in 0..cols {
            arrayLab[i as usize][j as usize] = laby.chars().nth(count).expect("IDK");
            count = count + 1;
            if arrayLab[i as usize][j as usize] == 'I'{
                startLab_x = i;
                startLab_y  = j;
            }
            if arrayLab[i as usize][j as usize] == 'X' {
                endLab_x = i;
                endLab_y = j;
            }
        }
    }
    println!("value : {}", arrayLab[1][0]);
    let mut node = Node {
        cost: startLab_x + startLab_y + endLab_x + endLab_y,
        dead_end: false,
        children: [None, None, None, None],
        x:startLab_x,
        y:startLab_y
    };
    let mut response = "".to_string();
    response = doYouKnowDaWay(node, response, arrayLab, startLab_x, startLab_y, endLab_x, endLab_y).to_string();





    /*//let mut message = str["Challenge"]["MD5HashCash"]["message"].to_string();
    let mut message = "The Isa's funny basket eats our nervous basket.".to_string();

    //let message = message[1..message.len() - 1].to_string();
    let mut find = 0;
    let mut seed = 1;
    let mut binary_value= "".to_string();
    let mut completeSeed = "".to_owned();
    while completeSeed.len() < 16-hex::encode( seed.to_string()).len() {
        completeSeed.push('0');
    }
    completeSeed.push_str(&*hex::encode(seed.to_string()));
    let mut val = md5::compute(completeSeed.clone() + &message); // a modifier
    //let momo = str["Challenge"]["MD5HashCash"]["complexity"].to_string().parse::<i32>().unwrap();
    let momo = "28".to_string().parse::<i32>().unwrap();
    while find < momo {
        while completeSeed.len() < 16-hex::encode( seed.to_string()).len() {
            completeSeed.push('0');
        }
        completeSeed.push_str(&*hex::encode(seed.to_string()));
        val = md5::compute(completeSeed.clone() + &message);
        binary_value = convert_to_binary_from_hex( &*format!("{:X}", val)).to_string();
        for i in 0..momo {
            if binary_value.chars().nth(i as usize).unwrap() == '0' {
                find = find+1
            }
        }
        if find < momo{
            seed = seed+1;
            find = 0;
            completeSeed = "".to_string();
        }
    }
    println!("val : {:X} and seed : {}", val, completeSeed);
    println!("binary : {}", binary_value);
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
    }*/
}
}*/
