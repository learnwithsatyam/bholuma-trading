use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct TokenResponse {
    status: String,
    request_token: String,
    action: String,
    #[serde(rename = "type")]
    type_: String
}

