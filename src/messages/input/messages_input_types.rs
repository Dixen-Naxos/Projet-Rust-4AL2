use serde::Deserialize;
use std::string::String;
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
    PlayerToKill(String),
    Exit,
    None
}

impl MessageInputType {

    pub fn match_type(&self, player_to_kill: String) -> MessageInputResult {

        let self_name: String = String::from("TEMA LA PATATE");

        return match self {
            MessageInputType::Welcome(welcome) => {
                println!("version : {}", welcome.version);
                MessageInputResult::MessageOutputType(MessageOutputType::Subscribe(Subscribe{ name: self_name }))
            },
            MessageInputType::Challenge(challenge) => {
                let challenge_answer = challenge.match_challenge();
                let challenge_result = ChallengeResult{
                    answer: challenge_answer,
                    next_target: player_to_kill
                };
                MessageInputResult::MessageOutputType(MessageOutputType::ChallengeResult(challenge_result))
            },
            MessageInputType::PublicLeaderBoard(players) => {
                let mut player_index = 0;

                for i in 1..players.len() {
                    if players[i].is_active && (!players[player_index].is_active || players[player_index].name == self_name || players[i].score > players[player_index].score ) {
                        player_index = i;
                    }
                }

                MessageInputResult::PlayerToKill(players[player_index].name.clone())
            },
            MessageInputType::SubscribeResult(result) => {
                result.display();
                MessageInputResult::None
            },
            MessageInputType::EndOfGame(_) => MessageInputResult::Exit,
            _ => MessageInputResult::None
        }
    }
}