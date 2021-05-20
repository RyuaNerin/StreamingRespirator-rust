use chrono::{DateTime, Utc};
use serde_derive::*;
use serde_json::Value;
use crate::core::utils::serde_datetime;


#[derive(Debug, Serialize)]
pub struct PacketEvent {
    pub event: String, // user_update
    #[serde(with = "serde_datetime")]
    pub created_at: DateTime<Utc>,
    pub target: Value,
    pub source: Value,
}

impl PacketEvent {
    pub fn new_user_update() -> Self {
        PacketEvent {
            event: "user_update".to_string(),
            created_at: Utc::now(),
            target: Value::default(),
            source: Value::default(),
        }
    }
}
