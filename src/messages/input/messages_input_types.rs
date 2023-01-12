use std::option::Option;
use serde::Deserialize;
use crate::{ChallengeResult, MessageOutputType, Subscribe};
use crate::messages::input::message_welcome::Welcome;
use crate::messages::input::message_challenge::ChallengeMessage;
use crate::messages::input::message_subscribe_result::SubscribeResult;
use crate::messages::input::message_challenge_timeout::ChallengeTimeout;
use crate::messages::input::message_public_leader_board::PublicPlayer;
use crate::messages::input::message_end_of_game::EndOfGame;
use crate::messages::input::message_round_summary::RoundSummary;

#[derive(Deserialize)]
pub enum MessageInputType {
    Welcome(Welcome),
    Challenge(ChallengeMessage),
    SubscribeResult(SubscribeResult),
    ChallengeTimeout(ChallengeTimeout),
    PublicLeaderBoard(Vec<PublicPlayer>),
    EndOfGame(EndOfGame),
    RoundSummary(RoundSummary)
}

pub enum MessageInputResult {
    MessageOutputType(MessageOutputType),
    Exit,
    None
}

impl MessageInputType {

    pub fn match_type(&self) -> MessageInputResult {
        return match self {
            MessageInputType::Welcome(welcome) => {
                println!("version : {}", welcome.version);
                MessageInputResult::MessageOutputType(MessageOutputType::Subscribe(Subscribe{ name: "TEMA LA PATATE".to_string() }))
            }
            MessageInputType::Challenge(challenge) => {
                let challenge_answer = challenge.match_challenge();
                let challenge_result = ChallengeResult{
                    answer: challenge_answer,
                    next_target: "TEMA LA PATATE".to_string()
                };
                MessageInputResult::MessageOutputType(MessageOutputType::ChallengeResult(challenge_result))
            }
            MessageInputType::SubscribeResult(result) => {
                result.display();
                MessageInputResult::None
            },
            MessageInputType::EndOfGame(endOfGame) => MessageInputResult::Exit,
            _ => MessageInputResult::None
        }
    }
}