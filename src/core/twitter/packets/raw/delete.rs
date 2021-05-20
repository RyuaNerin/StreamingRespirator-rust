use serde_derive::*;

#[derive(Debug, Serialize)]
pub struct PacketDelete {
    pub delete: DeleteItem,
}

#[derive(Debug, Serialize)]
pub struct DeleteItem {
    pub status: StatusItem,
}

#[derive(Debug, Serialize)]
pub struct StatusItem {
    pub id: u64,
    pub id_str: String,
    pub user_id: u64,
    pub user_id_str: String,
}
