use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct ApiIndex {
    server_version: String,
}

#[get("/api")]
async fn index() -> impl Responder {
    let obj = ApiIndex {
        server_version: "0.0-alpha".to_string(),
    };

    HttpResponse::Ok().json(obj)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
