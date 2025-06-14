use std::fs::{self, File};
use std::io::Write;
use actix_web::http::Error;
use serde_json;
use time::{OffsetDateTime, Duration};
use crate::models::AccessToken::AccessToken;

pub fn save_to_file(data: &AccessToken, filename: &str) -> std::io::Result<()> {
    let json = serde_json::to_string_pretty(data)?; // pretty or just `to_string`
    let mut file = File::create(filename)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

pub fn read_from_file(filename: &str) -> std::io::Result<AccessToken> {
    let json = fs::read_to_string(filename)?;
    let token_data: AccessToken = serde_json::from_str(&json)?;
    Ok(token_data)
}

pub fn get_access_token_validity() -> OffsetDateTime{
    // access token is valid only for 24 hours
    OffsetDateTime::now_utc() + Duration::hours(24) 
}

pub fn get_fresh_access_token(filename: &str) ->  Result<AccessToken, std::io::Error> {
    let token = read_from_file(filename);
    return token;
}
