use std::time::Instant;
use md5::{Digest, Md5};
use crate::challenges_compute::challenge::Challenge;
use crate::messages::input::challenges::hash_cash_input::Md5HashCashInput;
use crate::messages::output::challenges::hash_cash_output::MD5HashCashOutput;

pub struct Md5HashCash {
    input : Md5HashCashInput
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

            if isize::from_str_radix(&*binary_value, 2).unwrap() == 0 {
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
        let md5hash_cash_value: MD5HashCashOutput = MD5HashCashOutput {
            seed : u64::from_str_radix(&*complete_seed, 16).expect("Ta race"),
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
