use serde::Deserialize;

#[derive(Deserialize)]
pub enum SubscribeResult {
    Ok,
    Err(SubscribeError)
}

#[derive(Deserialize)]
pub enum SubscribeError {
    AlreadyRegistered,
    InvalidName
}

impl SubscribeResult {

    pub fn display(&self) {
        match self {
            SubscribeResult::Ok => println!("Successfully registered"),
            SubscribeResult::Err(error) => error.display()
        }
    }
}

impl SubscribeError {

    fn display(&self) {
        match self {
            SubscribeError::AlreadyRegistered => println!("Error during registration : AlreadyRegistered"),
            SubscribeError::InvalidName => println!("Error during registration : InvalidName")
        }
    }
}