use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};

use models::TokenResponse;
mod models;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/chat")]
async fn getChat() -> impl Responder {
    // get the chat data from db
    // serialise in json 
    // send to frontend
}

#[get("/")]
async fn getStockData() -> impl Responder {
    // fetch data from zerodha
    // serialise it in json
    //send frontend
}

#[post("/updateChat")]
async fn updateChat() -> impl Responder {
    // update the chat in db
}

#[post("/callback/zerodha")]
async fn updateAccessToken(query: web::Query<TokenResponse>) -> impl Responder{
    // capture the request token from request
    let requestToken = &query.request_token;
    // use apikey and request token to get the access token
    let accessToken = get_access_token(process.env.ZERODHA_API_KEY, requestToken, process.env.ZERODHA_API_SECRET);
    // store the access token along with time in a json file.
    match access_token_result {
        Ok(access_token) => {
            if let Err(e) = save_to_file(&access_token, "BholumaTrading.config.json") {
                return HttpResponse::InternalServerError()
                    .body(format!("Failed to save token: {}", e));
            }

            HttpResponse::Ok().body("✅ Access token saved successfully")
        }
        Err(e) => HttpResponse::BadRequest()
            .body(format!("❌ Failed to fetch access token: {}", e)),
    }
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
