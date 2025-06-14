use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenResponse {
    pub status: String,
    pub request_token: String,
    pub action: String,
    #[serde(rename = "type")]
    pub type_: String,
}
