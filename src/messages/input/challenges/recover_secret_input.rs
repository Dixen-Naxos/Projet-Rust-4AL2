use serde::Deserialize;

#[derive(Deserialize)]
pub struct RecoverSecretInput {
    pub word_count: usize,
    pub letters: String,
    pub tuple_sizes: Vec<usize>,
}

impl RecoverSecretInput {

    pub fn clone(&self) -> RecoverSecretInput {
        return RecoverSecretInput{
            word_count: self.word_count,
            letters: self.letters.clone(),
            tuple_sizes: self.tuple_sizes.clone()
        }
    }
}