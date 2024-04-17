mod auth;
mod schema;
mod models;
mod repository;

#[macro_use] extern crate rocket;

use rocket::serde::json::{Value, json, Json};
use rocket::response::status;
use rocket_sync_db_pools::database;
use auth::BasicAuth;
use models::RustaceanData;
use crate::repository::SqliteRepository;


#[database("sqlite")]
struct DbConn(diesel::SqliteConnection);

#[get("/rustaceans")]
async fn get_rustaceans(_auth: BasicAuth, db: DbConn) -> Value {
    db.run(|c| {
        let result = SqliteRepository::get_all_rustaceans(c)
            .expect("DB error while get all rustaceans");
        json!(result)
    }).await
}

#[get("/rustaceans/<id>")]
async fn view_rustacean(id: i32, _auth: BasicAuth, db: DbConn) -> Value {
    db.run(move |c| {
        let result = SqliteRepository::get_rustacean(c, id)
            .expect("DB error while get rustacean by id");
        json!(result)
    }).await
}

#[post("/rustaceans", format = "json", data = "<new_rustacean>")]
async fn create_rustacean(_auth: BasicAuth, db: DbConn, new_rustacean: Json<RustaceanData>) -> Value {
    db.run(|c| {
        let result = SqliteRepository::create_rustacean(c, new_rustacean.into_inner())
            .expect("DB error while inserting new rustacean");
        json!(result)
    }).await
}

#[put("/rustaceans/<id>", format = "json", data = "<new_rustacean>")]
async fn update_rustacean(id: i32, _auth: BasicAuth, db: DbConn, new_rustacean: Json<RustaceanData>) -> Value {
    db.run(move |c| {
        let result = SqliteRepository::update_rastacean(c, id, new_rustacean.into_inner())
            .expect("DB error while inserting new rustacean");
        json!(result)
    }).await
}

#[delete("/rustaceans/<id>")]
async fn delete_rustacean(id: i32, _auth: BasicAuth, db: DbConn) -> status::NoContent {
    db.run(move |c| {
        let result = SqliteRepository::delete_rustacean(c, id)
            .expect("DB error while deleting rustacean");
        json!(result)
    }).await;
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
