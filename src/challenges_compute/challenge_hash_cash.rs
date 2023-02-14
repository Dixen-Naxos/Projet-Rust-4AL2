use std::num::ParseIntError;
use std::time::Instant;
use std::convert::TryInto;
use md5::{Digest, Md5};
use crate::challenges_compute::challenge::Challenge;
use crate::messages::input::challenges::hash_cash_input::Md5HashCashInput;
use crate::messages::output::challenges::hash_cash_output::MD5HashCashOutput;

const S: [u32; 64] = [
    0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee,
    0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
    0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be,
    0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
    0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa,
    0xd62f105d, 0x2441453,  0xd8a1e681, 0xe7d3fbc8,
    0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed,
    0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,
    0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c,
    0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
    0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x4881d05,
    0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
    0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039,
    0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
    0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1,
    0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391,
];

const SHIFT_AMTS: [u32; 64] = [
    7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22,
    5, 9, 14, 20, 5, 9, 14, 20, 5, 9, 14, 20, 5, 9, 14, 20,
    4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23,
    6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21
];

pub struct Md5HashCash {
    input : Md5HashCashInput
}

impl Md5HashCash {

    fn md5_hash(message: &[u8]) -> String {

        let message_len_bits = (message.len() * 8) as u64;
        let mut message = message.to_vec();
        message.push(0x80);
        while message.len() % 64 != 56 {
            message.push(0);
        }
        let message_len_bits_bytes = message_len_bits.to_le_bytes();
        message.extend(&message_len_bits_bytes);
        let mut a0 = 0x67452301u32;
        let mut b0 = 0xefcdab89u32;
        let mut c0 = 0x98badcfeu32;
        let mut d0 = 0x10325476u32;
        for chunk in message.chunks(64) {
            let mut a = a0;
            let mut b = b0;
            let mut c = c0;
            let mut d = d0;
            let mut f: u32;
            let mut g: usize;
            let mut temp: u32;
            let mut w = [0u32; 16];
            for i in 0..16 {
                w[i] = u32::from_le_bytes(chunk[i*4..(i+1)*4].try_into().unwrap());
            }
            for i in 0..64 {
                if i < 16 {
                    f = (b & c) | (!b & d);
                    g = i;
                } else if i < 32 {
                    f = (d & b) | (!d & c);
                    g = (5*i + 1) % 16;
                } else if i < 48 {
                    f = b ^ c ^ d;
                    g = (3*i + 5) % 16;
                } else {
                    f = c ^ (b | !d);
                    g = (7*i) % 16;
                }
                temp = d;
                d = c;
                c = b;
                b = b.wrapping_add(
                    a.wrapping_add(f).wrapping_add(S[i]).wrapping_add(w[g])
                        .rotate_left(SHIFT_AMTS[i])
                );
                a = temp;
            }
            a0 = a0.wrapping_add(a);
            b0 = b0.wrapping_add(b);
            c0 = c0.wrapping_add(c);
            d0 = d0.wrapping_add(d);
        }
        let result = [a0, b0, c0, d0]
            .iter()
            .flat_map(|n| n.to_le_bytes().to_vec())
            .collect::<Vec<u8>>();
        format!("{:08X}{:08X}{:08X}{:08X}", result[0], result[1], result[2], result[3])
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
            val = Md5HashCash::md5_hash((complete_seed.clone() + &*self.input.message).as_bytes());
            let mut binary_value = convert_to_binary_from_hex( val.clone() ).to_string();
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
        let md5hash_cash_value: MD5HashCashOutput = MD5HashCashOutput {
            seed,
            hashcode : val
        };
        return md5hash_cash_value;
    }

    fn verify(&self, answer: &Self::Output) -> bool {
        todo!()
    }
}

fn convert_to_binary_from_hex(hex: String) -> String {
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
