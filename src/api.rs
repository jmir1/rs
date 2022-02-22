use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use std::io::Result;
use crate::db;
use db::GetResult;

#[get("/")]
pub async fn hello(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/insert")]
pub async fn insert(req: HttpRequest, req_body: String) -> impl Responder {
    let client = req.app_data::<&mongodb::sync::Client>().unwrap();
    let insert_result = db::insert(client, &req_body).get();
    println!("inserted with id {}", insert_result.inserted_id);
    HttpResponse::Ok().body(format!("inserted document {} with id {}", req_body, insert_result.inserted_id))
}

async fn manual_hello(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
pub async fn api(_client: &'static mongodb::sync::Client) -> Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(_client)
            .service(hello)
            .service(insert)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}