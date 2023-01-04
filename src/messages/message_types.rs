use serde::{Deserialize, Serialize};
use crate::messages::message_welcome::WelcomeMessage;

pub enum MessageType {
    Hello,
    Welcome(WelcomeMessage)
}