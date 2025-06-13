use serde::{Deserialize};

#[derive(Deserialize, Debug)]
struct TokenResponse {
    status: String,
    request_token: String,
    action: String,
    type: String
}
