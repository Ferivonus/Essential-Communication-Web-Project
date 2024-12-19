use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// Define the structure for form pages
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct FormPage {
    pub id: i32,
    pub form_name: String,
    pub page_url: String,
    pub created_at: DateTime<Utc>, // Using UTC for consistency
    pub updated_at: DateTime<Utc>,
    pub is_active: bool,
}
