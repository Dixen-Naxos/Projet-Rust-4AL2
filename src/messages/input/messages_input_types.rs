use serde::Deserialize;
use crate::messages::input::message_welcome::Welcome;
use crate::messages::input::message_challenge::Challenge;
use crate::messages::input::message_subscribe_result::SubscribeResult;
use crate::messages::input::message_challenge_timeout::ChallengeTimeout;
use crate::messages::input::message_public_leader_board::PublicPlayer;
use crate::messages::input::message_end_of_game::EndOfGame;
use crate::messages::input::message_round_summary::RoundSummary;

#[derive(Deserialize)]
pub enum MessageInputType {
    Hello,
    Welcome(Welcome),
    Challenge(Challenge),
    SubscribeResult(SubscribeResult),
    ChallengeTimeout(ChallengeTimeout),
    PublicLeaderBoardMessage(Vec<PublicPlayer>),
    EndOfGame(EndOfGame),
    RoundSummary(RoundSummary)
}
