use serde::Deserialize;
use crate::challenges_compute::challenge_hash_cash::Md5HashCash;
use crate::challenges_compute::challenge_nonogram::Nonogram;
use crate::challenges_compute::challenge_recover_secret::RecoverSecret;
use crate::challenges_compute::challenge::Challenge;
use crate::messages::input::challenges::hash_cash_input::Md5HashCashInput;
use crate::messages::input::challenges::nonogram_input::NonogramSolverInput;
use crate::messages::input::challenges::recover_secret_input::RecoverSecretInput;
use crate::messages::output::message_challenge_result::ChallengeAnswer;

#[derive(Deserialize)]
pub enum ChallengeMessage {
    MD5HashCash(Md5HashCashInput),
    RecoverSecret(RecoverSecretInput),
    NonogramSolver(NonogramSolverInput)
}

impl ChallengeMessage {

    pub fn match_challenge(&self) -> ChallengeAnswer {
        return match self {
            ChallengeMessage::MD5HashCash(input) => {
                let md5 = Md5HashCash::new((*input).clone());
                let answer = md5.solve();
                ChallengeAnswer::MD5HashCash(answer)
            }
            ChallengeMessage::RecoverSecret(input) => {
                let recovery_secret = RecoverSecret::new((*input).clone());
                let answer = recovery_secret.solve();
                ChallengeAnswer::RecoverSecret(answer)
            }
            ChallengeMessage::NonogramSolver(input) => {
                let nonogram = Nonogram::new((*input).clone());
                let answer = nonogram.solve();
                ChallengeAnswer::NonogramSolver(answer)
            }
        }
    }
}