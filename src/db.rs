use mongodb::{sync::Client, results::InsertOneResult};
use chrono::{Utc, Datelike};
use mongodb::bson::doc;
use std::env;
use std::error::Error;

pub type DbResult<T> = Result<T, Box<dyn Error>>;

pub trait GetResult<T> {
   fn get(self) -> T;
}

impl<T> GetResult<T> for DbResult<T> {
   fn get(self) -> T {
       match self {
           Ok(res) => { Ok(res) }
           Err(e) => { Err(std::io::Error::new(std::io::ErrorKind::Other, format!("{}", e))) }
       }.unwrap()
   }
}

pub fn connect() -> Client {
   // Load the MongoDB connection string from an environment variable:
   let client_uri =
      env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");
   // A Client is needed to connect to MongoDB:
   let client =
      mongodb::sync::Client::with_uri_str(client_uri).unwrap();
   // Print the databases in our MongoDB cluster:
   println!("Databases:");
   for name in client.list_database_names(None, None).unwrap() {
      println!("- {}", name);
   }
   client
}

pub fn insert(client: &Client, str: &String) -> DbResult<InsertOneResult> {
   let new_doc = doc! {
      "title": str,
      "year": Utc::now().year(),
      "attr1": str,
      "date": Utc::now(),
   };
   // Get the 'rs' collection from the 'rs' database:
   let rs = client.database("rs").collection("rs");
   let insert_result = rs.insert_one(new_doc.clone(), None)?;
   Ok(insert_result)
}
