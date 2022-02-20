use mongodb::{sync::Client, options::{ClientOptions, ResolverConfig}};
use chrono::{TimeZone, Utc};
use mongodb::bson::doc;
use std::env;
use std::error::Error;

pub fn connect() -> std::result::Result<Client, Box<dyn Error>> {
   // Load the MongoDB connection string from an environment variable:
   let client_uri =
      env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");
   // A Client is needed to connect to MongoDB:
   // An extra line of code to work around a DNS issue on Windows:
   let options =
      ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())?;
   let client = Client::with_options(options)?;
   // Print the databases in our MongoDB cluster:
   println!("Databases:");
   for name in client.list_database_names(None, None)? {
      println!("- {}", name);
   }
   Ok(client)
}

pub async fn insert(client: Client) -> Result<mongodb::results::InsertOneResult, Box<dyn Error>> {
   let new_doc = doc! {
      "title": "doc",
      "year": 2020,
      "attr1": "val1",
      "date": Utc.ymd(2020, 2, 7).and_hms(0, 0, 0),
   };
   // Get the 'rs' collection from the 'rs' database:
   let rs = client.database("rs").collection("rs");
   let insert_result = rs.insert_one(new_doc.clone(), None)?;
   Ok(insert_result)
}
