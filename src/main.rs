use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

// API Tweets
#[get("/tweets")]
async fn get_tweets() -> HttpResponse {
    let tweets = ["tweet 1: hello", "tweet 2: goodbye"];

    HttpResponse::Ok()
        .content_type("application/json")
        .json(tweets)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_tweets))
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}
