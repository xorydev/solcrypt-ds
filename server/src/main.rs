use actix_web::{dev::ServiceRequest, get, http::header, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[get("/")]
async fn index() -> impl Responder {
    "HAAAAIIII :33"
}

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

