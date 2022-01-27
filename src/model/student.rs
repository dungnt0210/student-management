use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Student {
    pub identifier: u16,
    pub name: String,
    pub class: String,
    pub score: u8
}

