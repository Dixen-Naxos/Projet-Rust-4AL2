use serde::Deserialize;

#[derive(Deserialize)]
pub struct PublicPlayer {
    name: String,
    stream_id: String,
    score: u8,
    steps: u8,
    is_active: bool,
    total_used_time: f32
}