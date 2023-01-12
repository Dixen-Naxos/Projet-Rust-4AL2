use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct RecoverSecretInput {
    pub word_count: usize,
    pub letters: String,
    pub tuple_sizes: Vec<usize>,
}
