use serde::Serialize;
use crate::challenges_compute::challenge_hash_cash::MD5HashCashValue;
use crate::challenges_compute::challenge_nonogram::NonogramSolverOutput;
use crate::challenges_compute::challenge_recover_secret::RecoverSecretOutput;

#[derive(Serialize)]
pub struct ChallengeResult {
    pub answer: ChallengeAnswer,
    pub next_target: String
}

#[derive(Serialize)]
pub enum ChallengeAnswer {
    MD5HashCash(MD5HashCashValue),
    Nonogram(NonogramSolverOutput),
    RecoverSecret(RecoverSecretOutput)
}