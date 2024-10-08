use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("hello")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/api/yarrak-çük-sik", web::get().to(hello)))
        .bind("127.0.0.1:3000")? // it can change
        .run()
        .await
}
