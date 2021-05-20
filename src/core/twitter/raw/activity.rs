use std::collections::HashMap;

use serde_json::Value;
use serde_derive::*;

use super::{twitter_status::TwitterStatus, twitter_user::TwitterUser};

#[derive(Debug, Serialize, Deserialize)]
pub struct Activity {
    pub action: String,
    pub max_position: u64,
    pub sources: Vec<TwitterUser>,
    pub targets: Value,
    pub target_objects: Vec<TwitterStatus>,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}
