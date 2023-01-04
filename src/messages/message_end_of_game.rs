use serde::{Deserialize, Serialize};
use crate::messages::message_public_leader_board::PublicPlayer;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EndOfGameMessage {
    end_of_game: EndOfGame
}

#[derive(Serialize, Deserialize)]
struct EndOfGame {
    leader_board: Vec<PublicPlayer>
}
