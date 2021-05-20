use chrono::{DateTime, Utc};
use serde_derive::*;

use crate::core::{twitter::TwitterUser, utils::serde_datetime};

#[derive(Debug, Serialize)]
pub struct PacketDirectMessage {
    pub direct_message: DirectMessageItem,
}


#[derive(Debug, Serialize)]
pub struct DirectMessageItem {
    pub id: u64,
    pub id_str: String,
    pub text: String,
    #[serde(with = "serde_datetime")]
    pub created_at: DateTime<Utc>,
    pub sender: TwitterUser,
    pub sender_id: u64,
    pub sender_screen_name: String,
    pub recipient: TwitterUser,
    pub recipient_id: u64,
    pub recipient_screen_name: String,
}
