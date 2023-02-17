use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Md5HashCashInput {
    pub complexity : i32,
    pub message : String
}
