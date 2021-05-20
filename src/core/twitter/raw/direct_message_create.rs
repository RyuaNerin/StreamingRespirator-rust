use serde_derive::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct DirectMessageNew {
    pub event: Event,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    #[serde(rename(serialize = "type_", deserialize = "type_"))]
    pub type_: String,
    pub message_create: MessagCreate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessagCreate {
    pub target: Target,
    pub message_data: MessageData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Target {
    pub recipient_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageData {
    pub text: String,
}
