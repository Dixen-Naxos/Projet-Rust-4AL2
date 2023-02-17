use serde::Deserialize;

#[derive(Deserialize)]
pub struct RoundSummary {
    challenge: String,
    chain: Vec<ReportedChallengeResult>
}

#[derive(Deserialize)]
struct ReportedChallengeResult {
    name: String,
    value: ChallengeResult
}

#[derive(Deserialize)]
enum ChallengeResult {
    Ok(ChallengeOk),
    Unreachable
}

#[derive(Deserialize)]
struct ChallengeOk {
    used_time: f32,
    next_target: String
}
