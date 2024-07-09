use actix_multipart::Multipart;
use actix_web::{web, App, HttpServer, HttpResponse, Error};
use futures_util::stream::StreamExt as _;
use std::io::Write;
use sanitize_filename::sanitize;

async fn save_file(mut payload: Multipart) -> Result<HttpResponse, Error> {
    while let Some(item) = payload.next().await {
        let mut field = item?;

        let content_disposition = field.content_disposition();
        let filename = if let Some(filename) = content_disposition.get_filename() {
            sanitize(filename)
        } else {
            "unnamed".to_string()
        };

        let filepath = format!("./uploads/{}", filename);

        let mut f = web::block(|| std::fs::File::create(filepath))
            .await?
            .map_err(actix_web::error::ErrorInternalServerError)?;

        while let Some(chunk) = field.next().await {
            let data = chunk?;
            f = web::block(move || f.write_all(&data).map(|_| f)).await??;
        }
    }

    Ok(HttpResponse::Ok().into())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::fs::create_dir_all("./uploads")?;

    HttpServer::new(|| {
        App::new()
            .route("/upload", web::post().to(save_file))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

