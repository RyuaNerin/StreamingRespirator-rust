use serde_json::{Map, Value};
use serde_derive::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct TwitterUser {
    pub id: u64,
    pub name: String,
    pub screen_name: String,
    pub profile_image_url: String,

    #[serde(flatten)]
    pub extra: Map<String, Value>,
}
