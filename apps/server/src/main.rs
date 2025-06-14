mod models;
mod utils;
mod zerodhaAuth;

use actix_web::{get, post, web::{self, get}, App, HttpResponse, HttpServer, Responder};
use models::TokenResponse::TokenResponse;
use models::AccessToken::AccessToken;
use serde_json::value;
use utils::commonUtilities::{save_to_file, read_from_file, get_access_token_validity};
use zerodhaAuth::kiteconnect::get_kite_connect;
use std::env::var;

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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
