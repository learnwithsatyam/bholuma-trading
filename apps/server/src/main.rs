use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};

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
async fun updateAccessToken() -> impl Responder{
    // capture the request token from request
    // use apikey and request token to get the access token
    // store the access token along with time in a json file.
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
