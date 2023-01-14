use serde::Serialize;
use crate::messages::output::challenges::hash_cash_output::MD5HashCashOutput;
use crate::messages::output::challenges::monstrous_maze_output::MonstrousMazeOutput;
use crate::messages::output::challenges::nonogram_output::NonogramSolverOutput;
use crate::messages::output::challenges::recover_secret_output::RecoverSecretOutput;

#[derive(Serialize)]
pub struct ChallengeResult {
    pub answer: ChallengeAnswer,
    pub next_target: String
}

#[derive(Serialize)]
pub enum ChallengeAnswer {
    MD5HashCash(MD5HashCashOutput),
    NonogramSolver(NonogramSolverOutput),
    RecoverSecret(RecoverSecretOutput),
    MonstrousMaze(MonstrousMazeOutput)
}