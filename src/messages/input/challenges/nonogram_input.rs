use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct NonogramSolverInput {
    pub rows: Vec<Vec<u32>>,
    pub cols: Vec<Vec<u32>>,
}
