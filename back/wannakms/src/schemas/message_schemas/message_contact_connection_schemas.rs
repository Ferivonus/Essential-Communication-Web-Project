use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct NewWireContact {
    pub nick: String,
    pub user_secret_key: String,
    pub other_side_secret_key: String,
    pub is_connection_possible: Option<bool>,
    pub connection_link: String,
}

// Struct for connection keys information
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessedConnectionKeysInfo {
    pub id: u16,
    pub user_secret_key: String,
    pub other_secret_key: String,
    pub is_connection_possible: bool,
    pub connection_link: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct ChangeConnectionPossibility {
    pub connection_link: String,
    pub new_is_connection_possible: bool,
}

#[derive(Serialize, Deserialize)]
pub struct ChangeConnectionKey {
    pub connection_link: String,
    pub user_secret_key: String,
    pub other_side_secret_key: String,
}

#[derive(Serialize, Deserialize)]
pub struct CheckConnectionKeys {
    pub user_secret_key: String,
    pub other_side_secret_key: String,
    pub connection_link: String,
}
