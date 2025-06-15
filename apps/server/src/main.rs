mod models;
mod utils;
mod zerodhaAuth;
extern crate kiteconnect;

use actix_web::{
    App, HttpResponse, HttpServer, Responder, body, get, post,
    web::{self, get},
};
use dotenv::dotenv;
use kiteconnect::ticker::{KiteTicker, KiteTickerHandler, WebSocketHandler};
use models::AccessToken::AccessToken;
use models::TokenResponse::TokenResponse;
use reqwest::Body;
use serde_json::value;
use std::{env::var, f64::consts::E, thread};
use utils::commonUtilities::{get_access_token_validity, get_fresh_access_token, save_to_file};
use zerodhaAuth::getHistoricalCandleData::fetch_100_candles;
use zerodhaAuth::kiteconnect::get_kite_connect;

use crate::{models::AccessToken::TokenData, zerodhaAuth::kiteTicker};

#[get("/zerodha/callback")]
async fn set_zerodha_access_token(data: web::Query<TokenResponse>) -> impl Responder {
    let mut token_response = TokenResponse {
        request_token: data.request_token.clone(),
        status: data.status.clone(),
        action: data.action.clone(),
        type_: data.type_.clone(),
    };
    let mut kiteconnect = get_kite_connect().unwrap();
    let api_secret = var("ZERODHA_API_SECRET").unwrap();
    let session =
        kiteconnect.generate_session(&token_response.request_token, api_secret.as_str().clone());

    match session {
        Ok(access_token) => {
            // save access token to file
            let access_token_data: AccessToken = serde_json::from_value(access_token).unwrap();

            let access_token_response = AccessToken {
                data: access_token_data.data,
                status: access_token_data.status,
            };

            match save_to_file(
                &access_token_response,
                var("ACCESS_TOKEN_CONFIG_FILE").unwrap().as_str(),
            ) {
                Ok(_) => HttpResponse::Ok().body(format!("saved access token.")),
                Err(_) => {
                    HttpResponse::InternalServerError().body(format!("Failed to save access token"))
                }
            }
        }
        Err(_) => {
            HttpResponse::InternalServerError().body(format!("Could not generate access token"))
        }
    }
}

#[get("/candle-data")]
async fn get_candle_data() -> impl Responder {
    let access_token_result =
        get_fresh_access_token(var("ACCESS_TOKEN_CONFIG_FILE").unwrap().as_str());
    match access_token_result {
        Ok(token) => {
            let candle_data = fetch_100_candles(&token.data.access_token, &"5633").await;
            match candle_data {
                Ok(value) => {
                    HttpResponse::Ok().json(value)
                }
                Err(_) => HttpResponse::InternalServerError().body("could not fetch candles"),
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("could not get access token"),
    }
}

fn start_ticker(api_key: String, access_token: String) {
    let mut ticker = KiteTicker::new(&api_key, &access_token);

    let custom_handler = kiteTicker::CustomHandler { count: 0 };

    println!("[ticker] Connecting with access token: {}", &access_token);

    // this blocks until disconnected
    match ticker.connect(custom_handler, None) {
        Ok(_) => println!("[ticker] Connection closed gracefully."),
        Err(e) => eprintln!("[ticker] Connection error: {:?}", e),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let api_key = std::env::var("ZERODHA_API_KEY")
        .unwrap_or_else(|error| panic!("ZERODHA API KEY NOT FOUND!!"));

    // ✅ Spawn the ticker in its own thread
    thread::spawn(move || {
        let mut connected: bool = false;
        loop {
            let access_token = get_fresh_access_token(
                var("ACCESS_TOKEN_CONFIG_FILE").unwrap_or_default().as_str(),
            );
            match access_token {
                Ok(token) => {
                    if (token.data.access_token == "" || connected) {
                        continue;
                    }
                    let mut ticker = KiteTicker::new(&api_key, &token.data.access_token);
                    let custom_handler = kiteTicker::CustomHandler { count: 0 };
                    match ticker.connect(custom_handler, None) {
                        Ok(_) => connected = true,
                        Err(e) => eprintln!("[ticker] Connection error: {:?}", e),
                    }
                }
                Err(e) => {
                    eprintln!("[ticker] no access token error: {:?}", e)
                }
            }

            //start_ticker(api_key.clone(), access_token.access_token);
            //println!("[ticker] Disconnected, retrying in 2s...");
            std::thread::sleep(std::time::Duration::from_secs(2));
        }
    });

    HttpServer::new(|| App::new().service(set_zerodha_access_token))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
