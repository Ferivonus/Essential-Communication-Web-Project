use serde::{Deserialize, Serialize};

// Define a struct to capture the incoming request payload
#[derive(Deserialize)]
pub struct Credentials {
    pub username: Option<String>,
    pub password: Option<String>,
}

// Define a struct for the authenticated user response
#[derive(Serialize)]
pub struct AuthenticatedUser {
    pub username: String,
}

#[derive(Deserialize)]
pub struct EnteringApplicationData {
    pub application_password: Option<String>,
    pub application_username: Option<String>,
}

#[derive(serde::Deserialize, sqlx::FromRow, Serialize)]
pub struct IsAccessibleCheckingData {
    pub connecting_user_name: String,
    pub user_password: String,
    pub other_password: String,
    pub link_the_user_want_connect: String,
}
