mod auth;
mod schema;
mod models;

#[macro_use] extern crate rocket;

use diesel::prelude::*;
use rocket::serde::json::{Value, json, Json};
use rocket::response::status;
use rocket_sync_db_pools::database;
use auth::BasicAuth;
use schema::rustaceans;
use models::{Rustacean, NewRustacean};


#[database("sqlite")]
struct DbConn(diesel::SqliteConnection);

#[get("/rustaceans")]
async fn get_rustaceans(_auth: BasicAuth, db: DbConn) -> Value {
    db.run(|c| {
        let result = rustaceans::table
            .order(rustaceans::id.desc())
            .limit(1000)
            .load::<Rustacean>(c)
            .expect("DB error while get all rustaceans");
        json!(result)
    }).await
}
#[get("/rustaceans/<id>")]
fn view_rustacean(id: i32, _auth: BasicAuth) -> Value {
    json!({"id": id, "name": "John Doe", "email": "john@doe.com"})
}
#[post("/rustaceans", format = "json", data = "<new_rustacean>")]
async fn create_rustacean(_auth: BasicAuth, db: DbConn, new_rustacean: Json<NewRustacean>) -> Value {
    db.run(|c| {
        let result = diesel::insert_into(rustaceans::table)
            .values(new_rustacean.into_inner())
            .execute(c).expect("DB error while inserting new rustacean");
        json!(result)
    }).await
}
#[put("/rustaceans/<id>", format = "json")]
fn update_rustacean(id: i32, _auth: BasicAuth) -> Value {
    json!({"id": id, "name": "John Doe", "email": "john@doe.com"})
}
#[delete("/rustaceans/<_id>")]
fn delete_rustacean(_id: i32, _auth: BasicAuth) -> status::NoContent {
    status::NoContent
}

#[catch(404)]
fn not_found() -> Value {
    json!("Not found")
}

#[catch(401)]
fn unauthorized() -> Value {
    json!("Unauthorized")
}

#[catch(422)]
fn unprocessable() -> Value {
    json!("Invalid entity. Probably some missing fields?")
}


#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", routes![
            get_rustaceans,
            view_rustacean,
            create_rustacean,
            update_rustacean,
            delete_rustacean,

        ])
        .register("/", catchers![
            not_found,
            unauthorized,
            unprocessable,
        ])
        .attach(DbConn::fairing())
        .launch()
        .await;
}
