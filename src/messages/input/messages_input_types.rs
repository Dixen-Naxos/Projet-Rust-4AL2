use serde::Deserialize;
use crate::{MessageOutputType, Subscribe};
use crate::messages::input::message_welcome::Welcome;
use crate::messages::input::message_challenge::Challenge;
use crate::messages::input::message_subscribe_result::SubscribeResult;
use crate::messages::input::message_challenge_timeout::ChallengeTimeout;
use crate::messages::input::message_public_leader_board::PublicPlayer;
use crate::messages::input::message_end_of_game::EndOfGame;
use crate::messages::input::message_round_summary::RoundSummary;

#[derive(Deserialize)]
pub enum MessageInputType {
    Welcome(Welcome),
    Challenge(Challenge),
    SubscribeResult(SubscribeResult),
    ChallengeTimeout(ChallengeTimeout),
    PublicLeaderBoardMessage(Vec<PublicPlayer>),
    EndOfGame(EndOfGame),
    RoundSummary(RoundSummary)
}

impl MessageInputType {

    pub fn match_type(&self) -> Option<MessageOutputType> {

        match self {
            MessageInputType::Welcome(welcome) => {
                println!("version : {}", welcome.version);
                return Option::from(MessageOutputType::Subscribe(Subscribe{ name: "TEMA LA PATATE".to_string() }));
            }
            MessageInputType::Challenge(_) => {}
            MessageInputType::SubscribeResult(result) => result.display(),
            _ => {}
        }

        None
    }
}