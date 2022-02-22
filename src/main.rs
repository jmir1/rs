#[macro_use]
extern crate lazy_static;
use std::io::Result;
mod db;
mod api;

fn main() -> Result<()> {
    lazy_static! {
        static ref CLIENT: mongodb::sync::Client = db::connect();
    }
    api::api(&CLIENT)
}
