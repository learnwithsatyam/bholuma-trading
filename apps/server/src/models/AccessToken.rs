use time::OffsetDateTime;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AccessToken{
    pub access_token : String,
    pub valid_till : OffsetDateTime,
}