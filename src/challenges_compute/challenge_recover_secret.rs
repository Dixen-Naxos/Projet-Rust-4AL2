use serde::{Deserialize, Serialize};
use crate::challenges_compute::challenge::Challenge;

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct RecoverSecretAnswer {
    recover_secret : RecoverSecretOutput
}

#[derive(Serialize)]
pub struct RecoverSecretOutput {
    pub secret_sentence: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RecoverSecret {
    pub recover_secret: RecoverSecretInput
}

#[derive(Deserialize)]
pub struct RecoverSecretInput {
    pub word_count: usize,
    pub letters: String,
    pub tuple_sizes: Vec<usize>,
}

impl RecoverSecret {

    fn insert(&self, result: String, index: usize, letters_index: usize) -> String {

        println!("-------- insert");
        let mut cnt: isize = -1;
        let mut i_cnt: isize = -1;
        let mut array: Vec<isize> = Vec::new();

        if index == self.recover_secret.tuple_sizes.len() {
            return result;
        }

        for _ in 0..self.recover_secret.tuple_sizes[index] {
            array.push(-1);
        }

        for i in 0..self.recover_secret.tuple_sizes[index] {
            for j in 0..result.len() {
                if self.recover_secret.letters.as_bytes()[letters_index + i] == result.as_bytes()[j] {
                    array[i] = j as isize;
                }
            }
        }

        println!("result = {}", result);
        println!("index = {}", index);
        for i in 0..array.len() {
            println!("{}", array[i]);
        }

        for i in 0..self.recover_secret.tuple_sizes[index] {
            if cnt >= array[i] && array[i] != -1 {
                //let mut s_return = String::from(self.recover_secret.letters.as_bytes()[letters_index + i] as char);
                //s_return.push(self.recover_secret.letters.as_bytes()[letters_index + i_cnt] as char);
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

        if index_pos == self.recover_secret.tuple_sizes[index] {
            return self.insert(result, index + 1, letters_index + self.recover_secret.tuple_sizes[index])
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
            result2 = format!("{}{}{}", p1, self.recover_secret.letters.as_bytes()[letters_index + index_pos] as char, p2);
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

        for i in 0..self.recover_secret.letters.len() {
            found = false;
            for j in 0..s_return.len() {
                if s_return.as_bytes()[j] == self.recover_secret.letters.as_bytes()[i] {
                    found = true;
                    break;
                }
            }

            if found {
                continue;
            }

            s_return.push(self.recover_secret.letters.as_bytes()[i] as char);
        }

        return s_return;
    }

    fn create_string_with_all_letters_double(&self) -> String {

        let mut s_return = String::from("");
        let mut found: bool;

        for i in 0..self.recover_secret.letters.len() {
            found = false;
            for j in 0..s_return.len() {
                if s_return.as_bytes()[j] == self.recover_secret.letters.as_bytes()[i] {
                    found = true;
                    break;
                }
            }

            if found {
                continue;
            }

            s_return.push(self.recover_secret.letters.as_bytes()[i] as char);
        }

        return s_return;
    }

    fn switch(&self) -> String {

        let mut s_return = self.create_string_with_all_letters();
        let mut s_swap;
        let mut ended;
        let mut letters_index = 0;
        let mut cnt: usize;
        let mut char_swap;

        while true {
            letters_index = 0;
            ended = true;

            for i in 0..self.recover_secret.tuple_sizes.len() {
                cnt = 0;

                for j in 0..self.recover_secret.tuple_sizes[i] {
                    for k in 0..s_return.len() {
                        if self.recover_secret.letters.as_bytes()[letters_index + j] == s_return.as_bytes()[k] {
                            if cnt > k {
                                s_swap = s_return.into_bytes();
                                char_swap = s_swap[k];
                                s_swap[k] = s_swap[cnt];
                                s_swap[cnt] = char_swap;
                                s_return = String::from_utf8(s_swap).unwrap();
                                ended = false;
                            } else {
                                cnt = k;
                            }
                            break;
                        }
                    }
                }

                letters_index += self.recover_secret.tuple_sizes[i];
            }

            if ended {
                break;
            }
        }

        return s_return;
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
            recover_secret: input
        }
    }

    fn solve(&self) -> Self::Output {

        /*let mut result = String::new();

        for i in 0..self.recover_secret.tuple_sizes[0] {
            result.push(self.recover_secret.letters.as_bytes()[i] as char);
        }
        self.insert(result, 1, self.recover_secret.tuple_sizes[0])*/

        return self.switch();
    }

    fn verify(&self, answer: &Self::Output) -> bool {
        false
    }
}