use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use serde_json::json;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json(json!({"msg": "Hello from Rust backend 🦀"}))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Rust backend running on http://0.0.0.0:5000/");
    HttpServer::new(|| App::new().service(hello))
        .bind(("0.0.0.0", 5000))?
        .run()
        .await
}

