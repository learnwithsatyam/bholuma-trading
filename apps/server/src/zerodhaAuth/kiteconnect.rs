extern crate kiteconnect;
extern crate serde_json as json;

use kiteconnect::connect::KiteConnect;
use std::error::Error;
use std::env::var;
use crate::utils::commonUtilities::read_from_file;

pub fn get_kite_connect() -> Result<KiteConnect, Box<dyn Error>> {
    let api_key = var("ZERODHA_API_KEY")?;
    let api_secret: String = var("ZERODHA_API_SECRET")?;
    let mut kiteconnect = KiteConnect::new(api_key.as_str(), api_secret.as_str().clone());
    Ok(kiteconnect)
}
