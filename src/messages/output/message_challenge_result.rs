use serde::Serialize;
use crate::challenges::challenge_hash_cash::MD5HashCashAnswer;
use crate::challenges::challenge_nonogram::NonogramAnswer;
use crate::challenges::challenge_recover_secret::RecoverSecretAnswer;

#[derive(Serialize)]
pub struct ChallengeResult {
    answer: ChallengeAnswer,
    next_target: String
}

#[derive(Serialize)]
enum ChallengeAnswer {
    MD5HashCashAnswer(MD5HashCashAnswer),
    NonogramAnswer(NonogramAnswer),
    RecoverSecretAnswer(RecoverSecretAnswer)
}