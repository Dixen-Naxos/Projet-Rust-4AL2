use std::{default, env};
use std::fmt::format;
use std::str;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};
use std::ops::Index;
use std::ptr::null;
use std::time::Instant;
use byteorder::{ByteOrder, BigEndian};
use serde_json::{json, Value};
use serde::{Deserialize, Serialize};
use std::sync::mpsc;
use std::thread;

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

#[derive(Serialize)]
struct RecoverSecretResult {
    ChallengeResult : RecoverSecretValue
}

#[derive(Serialize)]
struct RecoverSecretValue {
    answer : RecoverSecretAnswer,
    next_target : String
}

#[derive(Serialize)]
struct RecoverSecretAnswer {
    RecoverSecret : RecoverSecretOutput
}

#[derive(Deserialize)]
pub struct RecoverSecretInput {
    pub word_count: usize,
    pub letters: String,
    pub tuple_sizes: Vec<usize>,
}

#[derive(Serialize)]
pub struct RecoverSecretOutput {
    pub secret_sentence: String,
}

trait Challenge {

    type Input;
    type Output;

    fn name() -> String;

    fn new(input: Self::Input) -> Self;

    fn solve(&self) -> Self::Output;

    fn verify(&self, answer: &Self::Output) -> bool;
}

struct RecoverSecret {
    input: RecoverSecretInput
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

impl RecoverSecret {

    fn insert(&self, result: String, index: usize, letters_index: usize) -> String {

        println!("-------- insert");
        let mut cnt: isize = -1;
        let mut i_cnt: isize = -1;
        let mut array: Vec<isize> = Vec::new();

        if index == self.input.tuple_sizes.len() {
            return result;
        }

        for _ in 0..self.input.tuple_sizes[index] {
            array.push(-1);
        }

        for i in 0..self.input.tuple_sizes[index] {
            for j in 0..result.len() {
                if self.input.letters.as_bytes()[letters_index + i] == result.as_bytes()[j] {
                    array[i] = j as isize;
                }
            }
        }

        println!("result = {}", result);
        println!("index = {}", index);
        for i in 0..array.len() {
            println!("{}", array[i]);
        }

        for i in 0..self.input.tuple_sizes[index] {
            if cnt >= array[i] && array[i] != -1 {
                //let mut s_return = String::from(self.input.letters.as_bytes()[letters_index + i] as char);
                //s_return.push(self.input.letters.as_bytes()[letters_index + i_cnt] as char);
                return String::from("");
            }
            if array[i] != -1 {
                cnt = array[i];
                i_cnt = i as isize;
            }
        }

        self.insert_letters(result, index, letters_index, array, 0)
    }

    fn insert_letters(&self, mut result: String, index: usize, letters_index: usize, pos: Vec<isize>, index_pos: usize) -> String {

        println!("-------- letters");

        if index_pos == self.input.tuple_sizes[index] {
            return self.insert(result, index + 1, letters_index + self.input.tuple_sizes[index])
        }

        if pos[index_pos] != -1 {
            return self.insert_letters(result, index, letters_index, pos, index_pos + 1);
        }

        let mut min = 0;
        let mut max = result.len();

        if index_pos != 0 {
            min  = pos[index_pos - 1] as usize;
        }

        for i in (index_pos + 1)..pos.len() {
            if pos[i] != -1 {
                max = pos[i] as usize;
                break;
            }
        }

        let mut result2 = result.clone();

        for i in min..max {
            let mut pos2 = pos.clone();
            let (p1, p2) = result.split_at(i);
            result2 = format!("{}{}{}", p1, self.input.letters.as_bytes()[letters_index + index_pos] as char, p2);
            for j in i..pos2.len() {
                if pos2[j] != -1 {
                    pos2[j] += 1;
                }
            }
            pos2[index_pos] = i as isize;
            result2 = self.insert_letters(result2.clone(), index, letters_index, pos2, index_pos + 1);
            println!("r2 = {}", result2);
            if result2.len() != 0 {
                break;
            }
        }

        return result2;
    }

    fn create_string_with_all_letters(&self) -> String {

        let mut s_return = String::from("");
        let mut found: bool;

        for i in 0..self.input.letters.len() {
            found = false;
            for j in 0..s_return.len() {
                if s_return.as_bytes()[j] == self.input.letters.as_bytes()[i] {
                    found = true;
                    break;
                }
            }

            if found {
                continue;
            }

            s_return.push(self.input.letters.as_bytes()[i] as char);
        }

        return s_return;
    }

    fn switch(&self) -> String {

        let mut s_return = self.create_string_with_all_letters();
        let mut s_swap;
        let mut ended;
        let mut nb = 0;
        let mut letters_index = 0;
        let mut cnt: usize;
        let mut array: Vec<usize>;
        let mut char_swap;

        while true {
            nb = 0;
            letters_index = 0;

            for i in 0..self.input.tuple_sizes.len() {
                cnt = 0;
                array = Vec::new();

                for _ in 0..self.input.tuple_sizes[i] {
                    array.push(0);
                }

                for j in 0..self.input.tuple_sizes[i] {
                    for k in 0..s_return.len() {
                        if self.input.letters.as_bytes()[letters_index + j] == s_return.as_bytes()[k] {
                            array[j] = k;
                        }
                    }
                }

                ended = false;
                for j in 0..self.input.tuple_sizes[i] {
                    if cnt > array[j] {
                        s_swap = s_return.into_bytes();
                        char_swap = s_swap[array[j]];
                        s_swap[array[j]] = s_swap[cnt as usize];
                        s_swap[cnt as usize] = char_swap;
                        s_return = String::from_utf8(s_swap).unwrap();
                        ended = true;
                        break;
                    }
                    cnt = array[j];
                }

                if ended {
                    break;
                }

                letters_index += self.input.tuple_sizes[i];
                nb += 1;
            }

            if nb == self.input.tuple_sizes.len() {
                break;
            }
        }

        return s_return;
    }
}
/*
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
*/
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

impl Challenge for RecoverSecret {

    type Input = RecoverSecretInput;
    type Output = String;

    fn name () -> String {
        "Recover Secret".to_string()
    }

    fn new(input: Self::Input) -> Self {

        RecoverSecret {
            input
        }
    }

    fn solve(&self) -> Self::Output {

        /*let mut result = String::new();

        for i in 0..self.input.tuple_sizes[0] {
            result.push(self.input.letters.as_bytes()[i] as char);
        }
        self.insert(result, 1, self.input.tuple_sizes[0])*/

        return self.switch();
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
        // nonogram_solver(stream.try_clone().expect("Error cloning stream"));

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
