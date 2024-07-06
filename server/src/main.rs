use actix_web::{get, http::header, post, web, App, HttpResponse, HttpServer, Responder};
use actix_multipart::Multipart;

#[get("/")]
async fn index() -> impl Responder {
    "HAAAAIIII :33"
}

pub mod files {
    use std::io::Write;

    use actix_multipart::Multipart;
    use actix_web::web;
    use futures::{StreamExt, TryStreamExt};

    pub async fn save_file(mut payload: Multipart, file_path: String) -> Option<bool> {
        // iterate over multipart stream
        while let Ok(Some(mut field)) = payload.try_next().await {
            let content_type = field.content_disposition();
            //let filename = content_type.get_filename().unwrap();
            let filepath = format!(".{}", file_path);

            // File::create is blocking operation, use threadpool
            let mut f = web::block(|| std::fs::File::create(filepath))
                .await
                .unwrap();

            // Field in turn is stream of *Bytes* object
            while let Some(chunk) = field.next().await {
                let data = chunk.unwrap();
                // filesystem operations are blocking, we have to use threadpool
                f = web::block(move || f.as_ref().expect("malaka").write_all(&data).map(|_| f))
                    .await
                    .unwrap().expect("gamw thn mana sou");
            }
        }

        Some(true)
    }
}

#[post("/upload/")]
async fn file_upload(
    mut payload: Multipart
) -> std::io::Result<HttpResponse> {
  
    let upload_status = files::save_file(payload, "./thing.tar.xz".to_string()).await;

    match upload_status {
        Some(true) => {

            Ok(HttpResponse::Ok()
                .content_type("text/plain")
                .body("update_succeeded"))
        }
        _ => Ok(HttpResponse::BadRequest()
            .content_type("text/plain")
            .body("update_failed")),
    }
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

