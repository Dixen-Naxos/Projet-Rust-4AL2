use serde::Serialize;

#[derive(Serialize)]
pub struct NonogramSolverOutput {
    pub grid: String,
}