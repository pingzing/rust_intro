use rocket::serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct CreatePersonRequest {
    pub name: String,
    pub age: u16,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Person {
    pub id: usize,
    pub name: String,
    pub age: u16,
}
