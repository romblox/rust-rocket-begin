mod auth;
mod schema;
mod models;
mod repository;

#[macro_use] extern crate rocket;

use diesel::result::Error::NotFound;
use rocket::http::Status;
use rocket::serde::json::{Value, json, Json};
use rocket::response::status;
use rocket::response::status::Custom;
use rocket::{Build, Rocket};
use rocket::fairing::AdHoc;
use rocket_sync_db_pools::database;
use auth::BasicAuth;
use models::RustaceanData;
use crate::repository::SqliteRepository;


#[database("sqlite")]
struct DbConn(diesel::SqliteConnection);

#[get("/rustaceans")]
async fn get_rustaceans(_auth: BasicAuth, db: DbConn) -> Result<Value, Custom<Value>> {
    db.run(|c| {
        SqliteRepository::get_all_rustaceans(c)
            .map(|rustaceans| json!(rustaceans))
            .map_err(|err| Custom(Status::InternalServerError, json!(err.to_string())))
    }).await
}

#[get("/rustaceans/<id>")]
async fn view_rustacean(id: i32, _auth: BasicAuth, db: DbConn) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        SqliteRepository::get_rustacean(c, id)
            .map(|rustaceans| json!(rustaceans))
            .map_err(|err|
                match err {
                    NotFound => Custom(Status::NotFound, json!(err.to_string())),
                    _ => Custom(Status::InternalServerError, json!(err.to_string())),
                }
            )
    }).await
}

#[post("/rustaceans", format = "json", data = "<new_rustacean>")]
async fn create_rustacean(_auth: BasicAuth, db: DbConn, new_rustacean: Json<RustaceanData>) -> Result<Value, Custom<Value>> {
    db.run(|c| {
        SqliteRepository::create_rustacean(c, new_rustacean.into_inner())
            .map(|rustacean| json!(rustacean))
            .map_err(|err| Custom(Status::BadRequest, json!(err.to_string())))
    }).await
}

#[put("/rustaceans/<id>", format = "json", data = "<updated_rustacean>")]
async fn update_rustacean(id: i32, _auth: BasicAuth, db: DbConn, updated_rustacean: Json<RustaceanData>) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        SqliteRepository::update_rastacean(c, id, updated_rustacean.into_inner())
            .map(|rustacean| json!(rustacean))
            .map_err(|err|
                match err {
                    NotFound => Custom(Status::NotFound, json!(err.to_string())),
                    _ => Custom(Status::InternalServerError, json!(err.to_string())),
                })
    }).await
}

#[delete("/rustaceans/<id>")]
async fn delete_rustacean(id: i32, _auth: BasicAuth, db: DbConn) -> Result<status::NoContent, Custom<Value>> {
    db.run(move |c| {
        SqliteRepository::delete_rustacean(c, id)
            .map(|_| status::NoContent)
            .map_err(|err| Custom(Status::InternalServerError, json!(err.to_string())))
    }).await
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


async fn run_db_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
    const MIGRATIONS: EmbeddedMigrations = embed_migrations!();
    DbConn::get_one(&rocket)
        .await
        .expect("Unable to retrieve connection before apply migrations")
        .run(|c| {
            c.run_pending_migrations(MIGRATIONS).expect("Migrations failed");
        })
        .await;
    rocket
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
        .attach(AdHoc::on_ignite("Diesel migrations", run_db_migrations))
        .launch()
        .await;
}
