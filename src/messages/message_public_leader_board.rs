use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PublicLeaderBoardMessage {
    public_leader_board: Vec<PublicPlayer>
}

#[derive(Serialize, Deserialize)]
pub struct PublicPlayer {
    name: String,
    stream_id: String,
    score: u8,
    steps: u8,
    is_active: bool,
    total_time_used: f32
}