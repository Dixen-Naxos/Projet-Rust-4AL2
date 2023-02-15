use std::num::{ParseIntError, Wrapping};
use std::time::Instant;
use std::convert::TryInto;
use md5::{Digest, Md5};
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

    fn md5_hash(message: &[u8]) -> String {
        let mut h0: u32 = 0x67452301;
        let mut h1: u32 = 0xEFCDAB89;
        let mut h2: u32 = 0x98BADCFE;
        let mut h3: u32 = 0x10325476;

        let mut message = Md5HashCash::add_padding(message);

        for i in (0..message.len()).step_by(64) {
            let (mut a, mut b, mut c, mut d) = (h0, h1, h2, h3);

            for j in 0..64 {
                let f: u32;
                let g: usize;

                match j {
                    0..=15 => {
                        f = (b & c) | (!b & d);
                        g = j;
                    }
                    16..=31 => {
                        f = (d & b) | (!d & c);
                        g = (5 * j + 1) % 16;
                    }
                    32..=47 => {
                        f = b ^ c ^ d;
                        g = (3 * j + 5) % 16;
                    }
                    48..=63 => {
                        f = c ^ (b | !d);
                        g = (7 * j) % 16;
                    }
                    _ => unreachable!(),
                }

                let temp = d;
                d = c;
                c = b;
                b = Md5HashCash::left_rotate((a + f + K[j] + message[i + g] as u32).wrapping_add(S[j]), S[j]);
                a = temp;
            }

            h0 = h0.wrapping_add(a);
            h1 = h1.wrapping_add(b);
            h2 = h2.wrapping_add(c);
            h3 = h3.wrapping_add(d);
        }

        let mut result = [0; 16];
        result[..4].copy_from_slice(&h0.to_le_bytes());
        result[4..8].copy_from_slice(&h1.to_le_bytes());
        result[8..12].copy_from_slice(&h2.to_le_bytes());
        result[12..].copy_from_slice(&h3.to_le_bytes());
        let mut hash_string = format!("{:02X}", result[0]);
        for &byte in result[1..].iter() {
            hash_string.push_str(&format!("{:02X}", byte));
        }
        hash_string
    }

    fn add_padding(message: &[u8]) -> Vec<u8> {
        let original_length = message.len();
        let mut message = message.to_vec();
        message.push(0x80);

        while message.len() % 64 != 56 {
            message.push(0);
        }

        let bit_length = (original_length as u64 * 8).to_le_bytes();
        message.extend_from_slice(&bit_length);
        message
    }

    fn left_rotate(x: u32, n: u32) -> u32 {
        (x << n) | (x >> (32 - n))
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
        let now = Instant::now();

        let mut seed = 0;
        let mut complete_seed = "0000000000000000".to_string();
        let hexa = format!("{:X}", seed);
        complete_seed = complete_seed[0..16 - hexa.len()].to_string();
        complete_seed.push_str(&*hexa.to_string());
        let mut val;

        let momo = (&self).input.complexity;


        loop {

            complete_seed = "0000000000000000".to_string();
            let hexa = format!("{:X}", seed);
            complete_seed = complete_seed[0..16 - hexa.len()].to_string();
            complete_seed.push_str(&*hexa.to_string());
            let mut md5_hasher = Md5::new();
            md5_hasher.update(complete_seed.clone() + &*self.input.message);
            val = md5_hasher.finalize();
            let mut binary_value = convert_to_binary_from_hex( &*format!("{:X}", val) ).to_string();
            binary_value = binary_value[0..momo as usize].to_string();

            let prefix = match isize::from_str_radix(&*binary_value, 2) {
                Ok(prefix) => prefix,
                Err(_) => 0
            };
            if prefix == 0 {
                break
            }
            seed = seed+1;

            if now.elapsed().as_millis() > 1900 {
                println!("Viré");
                break
            }
        }
        let elapsed_time = now.elapsed();
        println!("Running boucle while took {} ms.", elapsed_time.as_millis());
        let seed = match u64::from_str_radix(&*complete_seed, 16) {
            Ok(seed) => seed,
            Err(_) => 0
        };
        println!("{}", Md5HashCash::md5_hash((complete_seed.clone() + &*self.input.message).as_bytes()));
        let md5hash_cash_value: MD5HashCashOutput = MD5HashCashOutput {
            seed,
            hashcode : format!("{:X}", val)
        };
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
