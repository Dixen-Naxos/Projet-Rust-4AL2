use serde::Deserialize;

#[derive(Deserialize)]
pub struct Md5HashCashInput {
    pub complexity : i32,
    pub message : String
}

impl Md5HashCashInput {

    pub fn clone(&self) -> Md5HashCashInput {
        return Md5HashCashInput{
            complexity: self.complexity,
            message: self.message.clone(),
        }
    }
}