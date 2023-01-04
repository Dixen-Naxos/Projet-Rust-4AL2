use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct SubscribeMessage {
    subscribe: Subscribe
}

#[derive(Serialize, Deserialize)]
struct Subscribe {
    name: String
}