use serde::Serialize;

#[derive(Serialize)]
pub struct RecoverSecretOutput {
    pub secret_sentence: String,
}