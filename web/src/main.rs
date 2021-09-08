mod person;
use std::time::Duration;

use person::{CreatePersonRequest, Person};
use rocket::{Error, response::status::BadRequest, serde::json::Json, tokio::time::sleep};

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/secondary")]
fn secondary() -> &'static str {
    "Hello, second!"
}

#[post("/", format = "json", data = "<person>")]
async fn post_person(person: Json<CreatePersonRequest>) -> Result<Json<Person>, BadRequest<()>> {
    sleep(Duration::from_secs(2)).await;    
    Ok(Json(Person {
        id: 1,
        age: person.0.age,
        name: person.0.name
    }))
}

#[rocket::main]
async fn main() -> Result<(), Error> {
    rocket::build()
        .mount("/", routes![index, secondary])
        .mount("/person", routes![post_person])
        .launch()
        .await
}


// Quick invocation for the person endpoint: 
//Invoke-RestMethod http://localhost:8000/person -Method Post -Body (@{name="Jimmy Joe";age=15} | ConvertTo-Json) -ContentType "application/json"