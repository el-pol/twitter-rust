use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

mod likes;
mod tweets;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_tweets))
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}
