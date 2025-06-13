extern crate kiteconnect;
extern crate serde_json as json;

mod commonUtilities;

use kiteconnect::connect::KiteConnect;
use std::error::Error;

fn get_kite_connect(requestToken: &str) -> Result<KiteConnect, Box<dyn Error>> {
    let mut kiteconnect = KiteConnect::new(process.env.ZERODHA_API_KEY, process.env.ZERODHA_API_SECRET);

    // Show login URL
    let login_url = kiteconnect.login_url();
    println!("Login at: {}", login_url);

    let accessTokenData = commonUtilities::read_from_file("BholumaTrading.config.json");

    // After manually logging in and getting the request token, use it:
    let session = kiteconnect.generate_session(requestToken, process.env.ZERODHA_API_SECRET)?;
    println!("Access Token Response: {:?}", session);

    Ok(kiteconnect)
}
