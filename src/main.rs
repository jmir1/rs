use std::io::Result;
mod db;
mod api;
use db::GetResult;

fn main() -> Result<()> {
    let client: mongodb::sync::Client = db::connect();
    let insert_result = db::insert(&client).get();
    println!("inserted with id {}", insert_result.inserted_id);
    api::api(&client)
}
