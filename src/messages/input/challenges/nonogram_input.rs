use serde::Deserialize;

#[derive(Deserialize)]
pub struct NonogramSolverInput {
    pub rows: Vec<Vec<u32>>,
    pub cols: Vec<Vec<u32>>,
}

impl NonogramSolverInput {

    pub fn clone(&self) -> NonogramSolverInput {
        return NonogramSolverInput{
            rows: self.rows.clone(),
            cols: self.cols.clone(),
        }
    }
}