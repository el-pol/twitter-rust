use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

async fn say_hi() -> impl Responder {
    HttpResponse::Ok().body("Hello!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/test", web::get().to(say_hi)))
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}
