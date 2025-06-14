mod models;
mod utils;
mod zerodhaAuth;
extern crate kiteconnect;

use actix_web::{get, post, web::{self, get}, App, HttpResponse, HttpServer, Responder};
use models::TokenResponse::TokenResponse;
use models::AccessToken::AccessToken;
use serde_json::value;
use utils::commonUtilities::{save_to_file, get_access_token_validity, get_fresh_access_token};
use zerodhaAuth::kiteconnect::get_kite_connect;
use std::{env::var, thread};
use kiteconnect::ticker::{KiteTicker, KiteTickerHandler, WebSocketHandler};
use zerodhaAuth::{*};

#[get("/zerodha/callback")]
async fn set_zerodha_access_token(data: web::Query<TokenResponse>) -> impl Responder {
    let mut token_response = TokenResponse{
        request_token : data.request_token.clone(),
        status : data.status.clone(),
        action : data.action.clone(),
        type_ : data.type_.clone(),        
    };
    let mut kiteconnect = get_kite_connect().unwrap();
    let api_secret = var("ZERODHA_API_SECRET").unwrap();
    let session = kiteconnect.generate_session(&token_response.request_token, api_secret.as_str().clone());

     match session {
        Ok(access_token) => {
            // save access token to file
            let access_token_response = AccessToken {
                access_token: access_token.to_string(),
                valid_till: get_access_token_validity(),
            };

            match save_to_file(&access_token_response, var("ACCESS_TOKEN_CONFIG_FILE").unwrap().as_str()) {
                Ok(_) => HttpResponse::Ok().body(format!("saved access token.")),
                Err(_) => HttpResponse::InternalServerError().body(format!("Failed to save access token")),
            }
        }
        Err(_) => {
            HttpResponse::InternalServerError().body(format!("Could not generate access token"))
        }
     }
}

fn start_ticker(api_key: String, access_token: String) {
    let mut ticker = KiteTicker::new(&api_key, &access_token);

    let custom_handler = kiteTicker::CustomHandler{ count : 0};

    println!("[ticker] Connecting with access token: {}", &access_token);

    // this blocks until disconnected
    ticker.connect(custom_handler, None);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let api_key = std::env::var("ZERODHA_API_KEY").unwrap_or_else(|error| panic!("ZERODHA API KEY NOT FOUND!!"));

    // ✅ Spawn the ticker in its own thread
    thread::spawn(move || {
        loop {
            let access_token = get_fresh_access_token(var("ACCESS_TOKEN_CONFIG_FILE").unwrap_or_default().as_str());
            start_ticker(api_key.clone(), access_token.access_token);
            println!("[ticker] Disconnected, retrying in 2s...");
            std::thread::sleep(std::time::Duration::from_secs(2));
        }
    });

    HttpServer::new(|| {
        App::new()
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
