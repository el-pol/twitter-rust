use actix_web::{get, post, web::Path, HttpResponse};

use crate::constants::APPLICATION_JSON;

// API Tweets
#[get("/tweets")]
pub async fn get_tweets() -> HttpResponse {
    let tweets = ["tweet 1: hello", "tweet 2: goodbye"];

    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(tweets)
}

#[post("/tweets")]
pub async fn create_tweet() -> HttpResponse {
    let new_tweet = "This is my new tweet";

    HttpResponse::Created()
        .content_type(APPLICATION_JSON)
        .json(new_tweet)
}

#[get("/tweets/{id}")]
pub async fn get_tweet_by_id(path: Path<(String,)>) -> HttpResponse {
    let tweet = format!("This is the tweet {:?}", path.0);

    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(tweet)
}
