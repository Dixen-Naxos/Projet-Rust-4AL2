use serde::{Deserialize, Serialize};
use crate::challenges::challenge_hash_cash::MD5HashCashAnswer;
use crate::challenges::challenge_nonogram::NonogramAnswer;
use crate::challenges::challenge_recover_secret::RecoverSecretAnswer;

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ChallengeResultMessage {
    challenge_result: ChallengeResult
}

#[derive(Serialize)]
struct ChallengeResult {
    answer: ChallengeAnswer,
    next_target: String
}

#[derive(Serialize)]
enum ChallengeAnswer {
    MD5HashCashAnswer(MD5HashCashAnswer),
    NonogramAnswer(NonogramAnswer),
    RecoverSecretAnswer(RecoverSecretAnswer)
}