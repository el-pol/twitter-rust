use actix_web::{App, HttpServer};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use std::env;

mod constants;
mod likes;
mod tweets;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not found");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder()
        .build(manager)
        .expect("Could not create the pool");

    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
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
