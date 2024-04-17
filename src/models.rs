use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Serialize, Deserialize};
use crate::schema::rustaceans;

#[derive(Serialize, Queryable)]
pub struct Rustacean {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Insertable, AsChangeset)]
#[diesel(table_name = rustaceans)]
pub struct RustaceanData {
    pub name: String,
    pub email: String,
}
