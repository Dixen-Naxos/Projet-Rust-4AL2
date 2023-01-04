use serde::Deserialize;

#[derive(Deserialize)]
pub enum SubscribeResult {
    Ok,
    Err(SubscribeError)
}

#[derive(Deserialize)]
enum SubscribeError {
    AlreadyRegistered,
    InvalidName
}