use std::num::{ParseIntError, Wrapping};
use std::time::Instant;
use std::convert::TryInto;
use std::fmt::LowerHex;
use std::sync::mpsc;
use std::thread;
use crate::challenges_compute::challenge::Challenge;
use crate::messages::input::challenges::hash_cash_input::Md5HashCashInput;
use crate::messages::output::challenges::hash_cash_output::MD5HashCashOutput;

const K: [u32; 64] = [
    0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee,
    0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
    0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be,
    0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
    0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa,
    0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8,
    0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed,
    0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,
    0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c,
    0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
    0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05,
    0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
    0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039,
    0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
    0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1,
    0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391,
];

const S: [u32; 64] = [
    7, 12, 17, 22,  7, 12, 17, 22,  7, 12, 17, 22,  7, 12, 17, 22,
    5,  9, 14, 20,  5,  9, 14, 20,  5,  9, 14, 20,  5,  9, 14, 20,
    4, 11, 16, 23,  4, 11, 16, 23,  4, 11, 16, 23,  4, 11, 16, 23,
    6, 10, 15, 21,  6, 10, 15, 21,  6, 10, 15, 21,  6, 10, 15, 21
];

pub struct Md5HashCash {
    input : Md5HashCashInput
}

impl Md5HashCash {

    fn md5(message: &str) -> String {
        let message = message.as_bytes();

        let mut a0: u32 = 0x67452301;
        let mut b0: u32 = 0xEFCDAB89;
        let mut c0: u32 = 0x98BADCFE;
        let mut d0: u32 = 0x10325476;

        let mut message = Self::pad_message(message);

        let mut buffer = [0u8; 64];
        for i in 0..(message.len() / 64) {
            for j in 0..16 {
                buffer[4 * j..4 * (j + 1)].copy_from_slice(&message[i * 64 + j * 4..i * 64 + (j + 1) * 4]);
            }

            let (mut a, mut b, mut c, mut d) = (a0, b0, c0, d0);

            for j in 0..64 {
                let f;
                let g;
                if j <= 15 {
                    f = (b & c) | ((!b) & d);
                    g = j;
                } else if j <= 31 {
                    f = (d & b) | ((!d) & c);
                    g = (5 * j + 1) % 16;
                } else if j <= 47 {
                    f = b ^ c ^ d;
                    g = (3 * j + 5) % 16;
                } else {
                    f = c ^ (b | (!d));
                    g = (7 * j) % 16;
                }

                let tmp = d;
                d = c;
                c = b;
                b = b.wrapping_add((a.wrapping_add(f).wrapping_add(K[j]).wrapping_add(u32::from_le_bytes(buffer[4 * g..4 * (g + 1)].try_into().unwrap()))).rotate_left(S[j]));
                a = tmp;
            }

            a0 = a0.wrapping_add(a);
            b0 = b0.wrapping_add(b);
            c0 = c0.wrapping_add(c);
            d0 = d0.wrapping_add(d);
        }

        let mut output = String::new();
        for h in &[a0, b0, c0, d0] {
            for b in &h.to_le_bytes() {
                output += &format!("{:02X}", b);
            }
        }

        output
    }

    fn pad_message(message: &[u8]) -> Vec<u8> {
        let initial_len = message.len();
        let bit_len = 8 * initial_len as u64;

        let mut padded_message = message.to_vec();
        padded_message.push(0x80);

        while (padded_message.len() * 8) % 512 != 448 {
            padded_message.push(0);
        }

        let len_bytes = bit_len.to_le_bytes();
        padded_message.extend_from_slice(&len_bytes);

        padded_message
    }

    fn found_solution(mut seed: i32, message: String, momo: i32) -> (i32, String) {

        let mut complete_seed;
        let mut val;
        let mut hexa;

        loop {

            complete_seed = "0000000000000000".to_string();
            hexa = format!("{:X}", seed);
            complete_seed = complete_seed[0..16 - hexa.len()].to_string();
            complete_seed.push_str(&*hexa.to_string());
            val = Self::md5(&*(complete_seed.clone() + &*message));
            let mut binary_value = convert_to_binary_from_hex( &*(val) ).to_string();
            binary_value = binary_value[0..momo as usize].to_string();

            let prefix = match isize::from_str_radix(&*binary_value, 2) {
                Ok(prefix) => prefix,
                Err(_) => 0
            };
            if prefix == 0 {
                return (seed, val);
            }
            seed = seed+1;
        }
    }
}

impl Challenge for Md5HashCash {
    type Input = Md5HashCashInput;
    type Output = MD5HashCashOutput;

    fn name() -> String {
        "Md5".to_string()
    }

    fn new(input: Self::Input) -> Self {
        Md5HashCash {input}
    }

    fn solve(&self) -> Self::Output {

        let (tx, rx) = mpsc::channel();

        for i in 0..3 {
            let tx1 = tx.clone();
            let message = self.input.message.clone();
            let momo = self.input.complexity.clone();
            thread::spawn(move || {
                let solution = Md5HashCash::found_solution(100000000 * i, message, momo);
                tx1.send(solution);
            });
        }

        let mut md5hash_cash_value = MD5HashCashOutput { seed: 0, hashcode: "".to_string() };

        for received in rx {
            md5hash_cash_value = MD5HashCashOutput {
                seed: received.0 as u64,
                hashcode : received.1
            };
            break;
        }


        return md5hash_cash_value;
    }

    fn verify(&self, answer: &Self::Output) -> bool {
        todo!()
    }
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
/*
#[test]
fn it_works() {
    let message = "aaa";
    let mut md5_hasher = Md5::new();
    md5_hasher.update(message);
    println!("{}", format!("{:X}", md5_hasher.finalize()));
    println!("{}", Md5HashCash::md5(message))
}
*/