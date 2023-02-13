use actix_web::{delete, get, post, web, App, HttpResponse, HttpServer, Path, Responder};

// API Tweets
#[get("/tweets")]
pub async fn get_tweets() -> HttpResponse {
    let tweets = ["tweet 1: hello", "tweet 2: goodbye"];

    HttpResponse::Ok()
        .content_type("application/json")
        .json(tweets)
}

#[post("/tweets")]
pub async fn create_tweet() -> HttpResponse {
    let new_tweet = "This is my new tweet";

    HttpResponse::Created()
        .content_type("application/json")
        .json(new_tweet)
}

#[get("/tweets/{id}")]
pub async fn get_tweet_by_id(path: Path<(String,)>) -> HttpResponse {
    let tweet = format!("This is the tweet {:?}", path.0);

    HttpResponse::Ok()
        .content_type("application/json")
        .json(tweet)
}
