use crate::constants::APPLICATION_JSON;
use actix_web::{delete, get, post, web::Path, HttpResponse};

#[get("/tweets/{id}/likes")]
pub async fn get_likes_by_tweet(path: Path<(String,)>) -> HttpResponse {
    let likes = 100;

    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(likes)
}

#[post("/tweets/{id}/likes")]
pub async fn like_tweet(path: Path<(String,)>) -> HttpResponse {
    let like = "Ok";

    HttpResponse::Created()
        .content_type(APPLICATION_JSON)
        .json(like)
}

#[delete("/tweets/{id}/likes")]
pub async fn remove_like(path: Path<(String,)>) -> HttpResponse {
    let result = "Ok";

    HttpResponse::Created()
        .content_type(APPLICATION_JSON)
        .await
        .unwrap()
}
