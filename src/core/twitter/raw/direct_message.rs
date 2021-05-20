use std::collections::HashMap;

use chrono::{DateTime, TimeZone, Utc};
use serde_derive::*;
use serde_json::Value;

use super::twitter_user::TwitterUser;

#[derive(Debug, Serialize, Deserialize)]
pub struct DirectMessage {
    pub user_inbox: DirectMessageUserItem,
    pub user_events: DirectMessageUserItem,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DirectMessageUserItem {
    pub conversations: Value,
    pub cursor: String,
    pub entries: Vec<DirectMessageEntry>,
    pub users: HashMap<String, TwitterUser>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DirectMessageEntry {
    pub message: Message,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub message_data: MessageData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageData {
    pub id: u64,
    pub time: u64,
    pub recipient_id: String,
    pub sender_id: String,
    pub text: String,
}

impl MessageData {
    pub fn get_create_at(self) -> DateTime<Utc> {
        let seconds = (self.time / 1000) as i64;
        let nanos = ((self.time % 1000) * 1_000_000) as u32;

        Utc.timestamp(seconds, nanos)
    }
}
