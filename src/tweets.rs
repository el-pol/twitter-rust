use actix_web::{delete, dev::Path, get, post, web, App, HttpResponse, HttpServer, Responder};

// API Tweets
#[get("/tweets")]
async fn get_tweets() -> HttpResponse {
    let tweets = ["tweet 1: hello", "tweet 2: goodbye"];

    HttpResponse::Ok()
        .content_type("application/json")
        .json(tweets)
}

#[post("/tweets")]
async fn create_tweet() -> HttpResponse {
    let new_tweet = "This is my new tweet";

    HttpResponse::Created()
        .content_type("application/json")
        .json(new_tweet)
}

#[get("/tweets/{id}")]
async fn get_tweet_by_id(path: Path<(String,)>) -> HttpResponse {
    let tweet = format!("This is the tweet {:?}", path.0);

    HttpResponse::Ok()
        .content_type("application/json")
        .json(tweet)
}

#[get("/tweets/{id}/likes")]
async fn get_likes_by_tweet(path: Path<(String,)>) -> HttpResponse {
    let likes = 100;

    HttpResponse::Ok()
        .content_type("application/json")
        .json(likes)
}

#[post("/tweets/{id}/likes")]
async fn like_tweet(path: Path<(String,)>) -> HttpResponse {
    let like = "Ok";

    HttpResponse::Created()
        .content_type("application/json")
        .json(like)
}

#[delete("/tweets/{id}/likes")]
async fn remove_like(path: Path<(String,)>) -> HttpResponse {
    let result = "Ok";

    HttpResponse::Created()
        .content_type("application/json")
        .await
        .unwrap()
}
