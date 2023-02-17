use serde::Deserialize;

#[derive(Deserialize)]
pub struct Welcome {
    pub version: u8,
}
