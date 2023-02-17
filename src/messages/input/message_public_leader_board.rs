use serde::Deserialize;

#[derive(Deserialize)]
pub struct PublicPlayer {
    pub name: String,
    pub stream_id: String,
    pub score: i8,
    pub steps: u8,
    pub is_active: bool,
    pub total_used_time: f32
}