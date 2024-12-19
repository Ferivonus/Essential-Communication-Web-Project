use serde::Deserialize;

#[derive(Deserialize)]
pub struct UpdateUserExtraInfo {
    pub user_nick: String,
    pub name: String,
    pub surname: String,
    pub age: u16,
    pub location: String,
    pub occupation: String,
    pub extra_info: String,
    pub new_nick: String,
}

#[derive(Deserialize)]
pub struct UpdatePhoneNumber {
    pub nick: String,
    pub phone_number: String,
}

#[derive(Deserialize)]
pub struct UpdateSocialMedia {
    pub nick: String,
    pub facebook: Option<String>,
    pub instagram: Option<String>,
    pub github: Option<String>,
    pub website: Option<String>,
    pub extra_social: Option<String>,
}

#[derive(Debug, serde::Serialize, Deserialize, sqlx::FromRow)]
pub struct ProcessedPhoneNumber {
    pub nick: String,
    pub phone_number: String,
}

#[derive(Debug, serde::Serialize, Deserialize, sqlx::FromRow)]
pub struct ProcessedSocialMedia {
    pub nick: String,
    pub facebook: Option<String>,
    pub instagram: Option<String>,
    pub github: Option<String>,
    pub website: Option<String>,
    pub extra_social: Option<String>,
}

#[derive(Debug, serde::Serialize, Deserialize, sqlx::FromRow)]
pub struct ProcessedPerson {
    pub nick: String,
    pub id: u16,
}

#[derive(Debug, serde::Serialize, Deserialize)]
pub struct FullDetailedPerson {
    pub id: String,
    pub nick: String,
    pub age: Option<u16>,
    pub location: Option<String>,
    pub occupation: Option<String>,
    pub extra_info: Option<String>,
    pub phone_number: String,
    pub facebook: Option<String>,
    pub instagram: Option<String>,
    pub github: Option<String>,
    pub website: Option<String>,
    pub extra_social: Option<String>,
}
