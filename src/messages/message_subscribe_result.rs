use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct SubscribeResultMessage {
    subscribe_result: SubscribeResult
}

#[derive(Serialize, Deserialize)]
enum SubscribeResult {
    Ok,
    Err(SubscribeError)
}

#[derive(Serialize, Deserialize)]
enum SubscribeError {
    AlreadyRegistered,
    InvalidName
}