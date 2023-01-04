use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RoundSummaryMessage {
    round_summary: RoundSummary
}

#[derive(Serialize, Deserialize)]
struct RoundSummary {
    challenge: String,
    chain: Vec<ReportedChallengeResult>
}

#[derive(Serialize, Deserialize)]
struct ReportedChallengeResult {
    name: String,
    value: ChallengeResult
}

#[derive(Serialize, Deserialize)]
enum ChallengeResult {
    Ok(ChallengeOk),
    Unreachable
}

#[derive(Serialize, Deserialize)]
struct ChallengeOk {
    used_time: f32,
    next_target: String
}
