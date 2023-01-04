use serde::Deserialize;

#[derive(Deserialize)]
pub struct ChallengeTimeout {
    message: String
}