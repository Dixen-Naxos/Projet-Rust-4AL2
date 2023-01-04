use serde::{Deserialize, Serialize};
use crate::challenges::challenge_hash_cash::Md5;
use crate::challenges::challenge_nonogram::Nonogram;
use crate::challenges::challenge_recover_secret::RecoverSecret;

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ChallengeMessage {
    challenge: Challenge
}

#[derive(Deserialize)]
enum Challenge {
    Md5(Md5),
    RecoverySecret(RecoverSecret),
    Nonogram(Nonogram)
}