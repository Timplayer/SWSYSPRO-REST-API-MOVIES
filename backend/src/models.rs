use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Movie {
    pub id: Option<i32>,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct MovieInput {
    pub name: String,
}
