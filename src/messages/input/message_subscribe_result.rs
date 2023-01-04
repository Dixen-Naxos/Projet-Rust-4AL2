use serde::Deserialize;

#[derive(Deserialize)]
pub enum SubscribeResult {
    Ok,
    Err(SubscribeError)
}

#[derive(Deserialize)]
pub enum SubscribeError {
    AlreadyRegistered,
    InvalidName
}