use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ChallengeTimeoutMessage {
    challenge_timeout: ChallengeTimeout
}

#[derive(Serialize, Deserialize)]
struct ChallengeTimeout {
    message: String
}