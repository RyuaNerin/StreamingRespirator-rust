use serde_derive::*;

use super::TwitterCredential;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub accounts: Vec<TwitterCredential>,
}
