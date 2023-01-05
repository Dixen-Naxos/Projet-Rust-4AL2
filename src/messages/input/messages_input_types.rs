use std::option::Option;
use serde::Deserialize;
use crate::{ChallengeResult, MessageOutputType, Subscribe};
use crate::challenges_compute::challenge::Challenge;
use crate::challenges_compute::challenge_hash_cash::Md5;
use crate::challenges_compute::challenge_recover_secret::RecoverSecret;
use crate::messages::input::challenges::recover_secret_input::RecoverSecretInput;
use crate::messages::input::message_welcome::Welcome;
use crate::messages::input::message_challenge::ChallengeMessage;
use crate::messages::input::message_subscribe_result::SubscribeResult;
use crate::messages::input::message_challenge_timeout::ChallengeTimeout;
use crate::messages::input::message_public_leader_board::PublicPlayer;
use crate::messages::input::message_end_of_game::EndOfGame;
use crate::messages::input::message_round_summary::RoundSummary;
use crate::messages::output::challenges::recover_secret_output::RecoverSecretOutput;
use crate::messages::output::message_challenge_result::ChallengeAnswer;

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

impl MessageInputType {

    pub fn match_type(&self) -> Option<MessageOutputType> {
        match self {
            MessageInputType::Welcome(welcome) => {
                println!("version : {}", welcome.version);
                return Option::from(MessageOutputType::Subscribe(Subscribe{ name: "TEMA LA PATATE".to_string() }));
            }
            MessageInputType::Challenge(challenge) => {
                let challenge_answer = challenge.match_challenge();
                let challenge_result = ChallengeResult{
                    answer: challenge_answer,
                    next_target: "TEMA LA PATATE".to_string()
                };
                return Option::from(MessageOutputType::ChallengeResult(challenge_result))
            }
            MessageInputType::SubscribeResult(result) => result.display(),
            _ => {}
        }

        None
    }
}