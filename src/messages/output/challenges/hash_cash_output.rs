use serde::Serialize;

#[derive(Serialize)]
pub struct MD5HashCashOutput {
    pub seed : u64,
    pub hashcode : String
}