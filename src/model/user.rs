use crate::{Result};
use serde::{Deserialize, Serialize};
use crate::helper::db;
#[derive(Serialize,Deserialize, Debug)]
pub struct User {
    pub username: String,
    pub password: String,
}

impl User {
    pub async fn gets() -> Result<Vec<User>> {
        db::DB.database.collection("users").
    }
}