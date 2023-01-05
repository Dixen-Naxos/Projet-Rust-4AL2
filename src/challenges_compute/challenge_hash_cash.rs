use std::time::Instant;
use md5::Digest;
use crate::challenges_compute::challenge::Challenge;
use crate::messages::input::challenges::hash_cash_input::Md5HashCashInput;
use crate::messages::output::challenges::hash_cash_output::MD5HashCashOutput;

pub struct Md5 {
    input : Md5HashCashInput
}

impl Challenge for Md5 {
    type Input = Md5HashCashInput;
    type Output = MD5HashCashOutput;

    fn name() -> String {
        "Md5".to_string()
    }

    fn new(input: Self::Input) -> Self {
        Md5 {input}
    }

    fn solve(&self) -> Self::Output {
        let now = Instant::now();
        //let mut message = str["Challenge"]["MD5HashCash"]["message"].to_string();
        let message = &self.input.message;
        //let mut message = "The Isa's funny basket eats our nervous basket.".to_string();

        let message = message[1..message.len() - 1].to_string();
        let mut seed = 0;
        //let mut binary_value= "".to_string();
        let mut complete_seed = "0000000000000000".to_string();
        /*while complete_seed.len() < 16-hex::encode( seed.to_string()).len() {
            complete_seed.push('0');
        }*/
        let hexa = hex::encode(seed.to_string());
        complete_seed = complete_seed[0..16 - hexa.len()].to_string();
        complete_seed.push_str(&*hexa.to_string());
        let mut val : Digest; // a modifier
        //let momo = str["Challenge"]["MD5HashCash"]["complexity"].to_string().parse::<i32>().unwrap();
        let momo = (&self).input.complexity;
        //let momo = "16".to_string().parse::<i32>().unwrap();

        loop {
            //while find < momo {
            complete_seed = "0000000000000000".to_string();
            /*while complete_seed.len() < 16-hex::encode( seed.to_string()).len() {
                 complete_seed.push('0');
             }*/
            let hexa = hex::encode(seed.to_string());
            complete_seed = complete_seed[0..16 - hexa.len()].to_string();
            complete_seed.push_str(&*hexa.to_string());
            val = md5::compute(complete_seed.clone() + &message);
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
        let md5hash_cash_value: MD5HashCashOutput = MD5HashCashOutput {
            //seed : "0x".to_string()+ &*complete_seed,
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
