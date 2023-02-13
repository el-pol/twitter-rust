use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

mod likes;
mod tweets;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(tweets::get_tweets)
            .service(tweets::create_tweet)
            .service(tweets::get_tweet_by_id)
            .service(likes::get_likes_by_tweet)
            .service(likes::like_tweet)
            .service(likes::remove_like)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
