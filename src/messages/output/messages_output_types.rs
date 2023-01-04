use serde::Serialize;
use crate::messages::output::message_subscribe::Subscribe;
use crate::messages::output::message_challenge_result::ChallengeResult;

#[derive(Serialize)]
pub enum MessageOutputType {
    Hello,
    Subscribe(Subscribe),
    ChallengeResult(ChallengeResult)
}
