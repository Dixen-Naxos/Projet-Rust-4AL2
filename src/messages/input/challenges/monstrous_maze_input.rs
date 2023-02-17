use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct MonstrousMazeInput {
    pub grid: String,
    pub endurance: u32,
}