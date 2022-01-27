use mongodb::bson::{self, doc, Bson, Client, ClientOptions, Database};

let client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
let client = Client::with_options(client_options)?;
pub let db = client.database("student_management");

#[derive(Clone, Debug)]
pub struct DB {
    pub database: Database
}

impl DB {
    pub async fn init() -> Result<Self> {
        let mut client_options = ClientOptions::parse("mongodb://127.0.0.1:27017").await?;
        client_options.app_name = Some("student_management".to_string());
        let client = Client::with_options(client_options)?;
        let database = client.database("student_management");
        Ok(Self {
            database: database
        })
    }
}