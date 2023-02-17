use serde::Deserialize;
use crate::messages::input::message_public_leader_board::PublicPlayer;

#[derive(Deserialize)]
pub struct EndOfGame {
    leader_board: Vec<PublicPlayer>
}
