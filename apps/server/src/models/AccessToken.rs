use time::OffsetDateTime;
use serde::{Serialize, Deserialize};

// #[derive(Debug, Serialize, Deserialize)]
// pub struct AccessToken {
//     pub access_token: String,
//     pub valid_till: OffsetDateTime
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessToken {
    pub data: TokenData,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenData {
    pub access_token: String,
    pub api_key: String,
    pub avatar_url: Option<String>,
    pub broker: String,
    pub email: String,
    pub enctoken: String,
    pub exchanges: Vec<String>,
    pub login_time: String,
    pub meta: Meta,
    pub order_types: Vec<String>,
    pub products: Vec<String>,
    pub public_token: String,
    pub refresh_token: String,
    pub user_id: String,
    pub user_name: String,
    pub user_shortname: String,
    pub user_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
    pub demat_consent: String,
}
