use serde::Deserialize;
use crate::challenges_compute::challenge_hash_cash::Md5HashCashInput;
use crate::challenges_compute::challenge_nonogram::NonogramSolverInput;
use crate::challenges_compute::challenge_recover_secret::RecoverSecretInput;

#[derive(Deserialize)]
pub enum ChallengeMessage {
    MD5HashCash(Md5HashCashInput),
    RecoverSecret(RecoverSecretInput),
    Nonogram(NonogramSolverInput)
}