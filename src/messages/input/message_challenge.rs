use serde::Deserialize;
use crate::challenges::challenge_hash_cash::Md5;
use crate::challenges::challenge_nonogram::Nonogram;
use crate::challenges::challenge_recover_secret::RecoverSecret;

#[derive(Deserialize)]
pub enum Challenge {
    Md5(Md5),
    RecoverySecret(RecoverSecret),
    Nonogram(Nonogram)
}