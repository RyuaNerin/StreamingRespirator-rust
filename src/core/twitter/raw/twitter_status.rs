use serde_json::{Map, Value};
use serde_derive::*;

use super::twitter_user::TwitterUser;

#[derive(Debug, Serialize, Deserialize)]
pub struct TwitterStatus {
    pub display_text_range: Vec<u16>,
    pub id: u64,
    pub user: TwitterUser,
    pub full_text: Option<String>,
    pub text: Option<String>,
    pub retweeted_status: Option<Box<TwitterStatus>>,
    pub quoted_status: Option<Box<TwitterStatus>>,

    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

impl TwitterStatus {
    pub fn get_full_text(self) -> Option<String> {
        match self.full_text {
            Some(x) => Some(x.clone()),
            None => match self.text {
                Some(x) => Some(x.clone()),
                None => None,
            },
        }
    }
}
