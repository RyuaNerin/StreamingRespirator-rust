use serde_derive::*;

#[derive(Debug, Serialize)]
pub struct PacketFriends {
    pub friends: Vec<u64>,
}
