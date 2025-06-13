use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenResponse {
    status: String,
    request_token: String,
    action: String,
    #[serde(rename = "type")]
    type_: String
}

