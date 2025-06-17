use std::fs::{self, File};
use std::io::Write;
use actix_web::http::Error;
use serde_json;
use time::{OffsetDateTime, Duration, Weekday, UtcOffset};
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

pub fn is_market_open_in_india() -> bool {
    // India Standard Time is UTC+5:30
    let ist_offset = UtcOffset::from_hms(5, 30, 0).unwrap();
    let now_ist = OffsetDateTime::now_utc().to_offset(ist_offset);

    let weekday = now_ist.weekday();
    let hour = now_ist.hour();
    let minute = now_ist.minute();

    // Market is closed on Saturday and Sunday
    if matches!(weekday, Weekday::Saturday | Weekday::Sunday) {
        return false;
    }

    // Convert time to minutes since midnight
    let total_minutes: u128 = (hour as u128) * 60 + (minute as u128);

    let market_open: u128 = 9 * 60 + 15;   // 9:15 AM
    let market_close: u128 = 15 * 60 + 30; // 3:30 PM

    total_minutes >= market_open && total_minutes <= market_close
}
