use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::io::{Error, ErrorKind, Result};
mod db;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> Result<()> {
    let client = match db::connect() {
        Ok(client) => { Ok(client) }
        Err(_e) => { Err(Error::new(ErrorKind::Other, "oh no!")) }
    }?;
    let insert_result = match db::insert(client).await {
        Ok(res) => { Ok(res) }
        Err(_e) => { Err(Error::new(ErrorKind::Other, "oh no!")) }
    }?;
    println!("inserted with id {}", insert_result.inserted_id);
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
