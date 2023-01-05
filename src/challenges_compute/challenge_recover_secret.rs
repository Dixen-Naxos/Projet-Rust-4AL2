use crate::challenges_compute::challenge::Challenge;
use crate::messages::input::challenges::recover_secret_input::RecoverSecretInput;
use crate::messages::output::challenges::recover_secret_output::RecoverSecretOutput;

pub struct RecoverSecret {
    pub input: RecoverSecretInput
}

impl RecoverSecret {

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
        let mut letters_index: usize;
        let mut cnt: usize;
        let mut char_swap;

        loop {
            letters_index = 0;
            ended = true;

            for i in 0..self.input.tuple_sizes.len() {
                cnt = 0;

                for j in 0..self.input.tuple_sizes[i] {
                    for k in 0..s_return.len() {
                        if self.input.letters.as_bytes()[letters_index + j] == s_return.as_bytes()[k] {
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

                letters_index += self.input.tuple_sizes[i];
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
    type Output = RecoverSecretOutput;

    fn name () -> String {
        "Recover Secret".to_string()
    }

    fn new(input: Self::Input) -> Self {
        RecoverSecret {input}
    }

    fn solve(&self) -> Self::Output {

        return RecoverSecretOutput {
            secret_sentence: self.switch()
        };
    }

    fn verify(&self, answer: &Self::Output) -> bool {
        todo!()
    }
}