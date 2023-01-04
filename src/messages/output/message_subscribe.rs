use serde::Serialize;

#[derive(Serialize)]
pub struct Subscribe {
    pub name: String
}