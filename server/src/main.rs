use actix_web::{dev::ServiceRequest, get, http::header, post, web, App, HttpResponse, HttpServer, Responder};
use prisma_client_rust::PrismaClient;
use serde::Deserialize;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
#[allow(unused, dead_code)]
mod prisma;

#[get("/")]
async fn index() -> impl Responder {
    "HAAAAIIII :33"
}

#[derive(Deserialize)]
struct ClientRegisterRequest {
    ip: String,
    hostname: String
}

#[derive(Deserialize)]
struct SetClientPaidRequest { password: String }

#[post("/client/register/{id}")]
async fn register_client(path: web::Path<u32>, info: web::Json<ClientRegisterRequest>) -> impl Responder {
    let db_client = match prisma::PrismaClient::_builder().build().await {
        Ok(client) => client,
        Err(err) => {
            eprintln!("Error building Prisma Client: {:?}", err);
            return HttpResponse::InternalServerError().body("Internal Server Error");
        }
    };
    
    let client = match db_client.client().create(
        format!("client_{}", path),
        info.ip.to_string(),
        info.hostname.to_string(),
        false,
        vec![]
    ).exec().await {
        Ok(client) => client,
        Err(err) => {
            eprintln!("Error building Prisma Client: {:?}", err);
            return HttpResponse::InternalServerError().body("Internal Server Error");
        }
    };
    HttpResponse::Ok().json(client)
}

#[get("/client/{id}/ispaid")]
async fn get_client_payment(path: web::Path<u32>) -> impl Responder {
    let db_client = match prisma::PrismaClient::_builder().build().await {
        Ok(client) => client,
        Err(err) => {
            eprintln!("Error building Prisma Client: {:?}", err);
            return HttpResponse::InternalServerError().body("Internal Server Error");
        }
    };
    
    let client = match db_client.client()
        .find_unique(prisma::client::id::equals(path.to_string()))
        .exec()
        .await
        .unwrap() {
            Some(client) => client,
            None => {
                eprintln!("Couldn't find client");
                return HttpResponse::NotFound().body("Not Found");
            }
    };

    HttpResponse::Ok().body(client.paid.to_string())
}

#[post("/client/{id}/setpaid")]
pub async fn set_client_paid(path: web::Path<String>, info: web::Json<SetClientPaidRequest>) -> impl Responder {

    let mut dotenv = File::open(".env").await.unwrap();
    let mut dotenv_contents = String::new();
    match dotenv.read_to_string(&mut dotenv_contents).await {
        Ok(result) => result,
        Err(err) => {
            eprintln!("Encountered error: {:?}", err);
            return HttpResponse::InternalServerError().body("Internal Server Error");
        },
    };

    let password = dotenv_contents
        .lines()
        .filter(|line| line.starts_with("PASSWORD="))
        .next()
        .unwrap()
        .split("=")
        .last()
        .unwrap(); // don't shout at me for the unwraps, your goddamn fault if you fucked the .env
                   // file
    
    if path.to_string() != password { return HttpResponse::Forbidden().body("Forbidden") }

    let db_client = match prisma::PrismaClient::_builder().build().await {
        Ok(client) => client,
        Err(err) => {
            eprintln!("Error creating Prisma Client: {:?}", err);
            return HttpResponse::InternalServerError().body("Internal Server Error");
        },
     };

    let mut client = match db_client
        .client()
        .find_first(vec![prisma::client::id::equals(path.to_string())])
        .exec()
        .await
        .unwrap() {
            Some(client) => client,
            None => {
                eprintln!("Couldn't find client!");
                return HttpResponse::NotFound().body("Not Found");
            },
    };

    client.paid = true;

    HttpResponse::Ok().body("Ok")
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

