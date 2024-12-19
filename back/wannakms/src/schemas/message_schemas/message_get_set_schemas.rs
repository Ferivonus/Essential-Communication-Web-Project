use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Serialize)]
pub struct Message {
    pub id: u16,
    pub sender: String,
    pub receiver: String,
    pub content: String,
    pub timestamp: DateTime<Utc>, // using chrono for timestamp
    pub close_one_point: Option<String>,
    pub connected: Option<String>,
    pub has_attachment: bool,
}

#[derive(Deserialize)]
pub struct NewMessage {
    pub sender: String,
    pub receiver: String,
    pub content: String,
    pub close_one_point: Option<String>,
    pub connected: Option<String>,
    pub has_attachment: bool,
}

#[derive(Serialize)]
pub struct MessageResponse {
    pub id: u16,
    pub sender: String,
    pub receiver: String,
    pub content: String,
    pub timestamp: String, // string for JSON response
    pub close_one_point: Option<String>,
    pub connected: Option<String>,
}

#[derive(Deserialize)]
pub struct NewFile {
    pub file_name: String,
    pub file_type: String,
    pub file_size: i64,
    pub file_data: Vec<u8>, // Binary data
}

#[derive(Serialize)]
pub struct FileResponse {
    pub id: i16,
    pub file_name: String,
    pub file_type: String,
    pub file_size: i64,
    pub timestamp: String,
}

impl Message {
    pub fn to_response(&self) -> MessageResponse {
        MessageResponse {
            id: self.id,
            sender: self.sender.clone(),
            receiver: self.receiver.clone(),
            content: self.content.clone(),
            timestamp: self.timestamp.to_rfc3339(), // Convert to string in RFC 3339 format
            close_one_point: self.close_one_point.clone(),
            connected: self.connected.clone(),
        }
    }
}
