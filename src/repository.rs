use diesel::QueryResult;
use crate::models::{Rustacean, RustaceanData};
use crate::schema::rustaceans;

use diesel::prelude::*;

pub struct SqliteRepository;

impl SqliteRepository {
    fn last_inserted_id(c: &mut SqliteConnection) -> QueryResult<i32> {
        rustaceans::table.select(rustaceans::id).order(rustaceans::id.desc()).first(c)
    }

    pub fn get_rustacean(c: &mut SqliteConnection, id: i32) -> QueryResult<Rustacean> {
        rustaceans::table.find(id).get_result::<Rustacean>(c)
    }

    pub fn get_all_rustaceans(c: &mut SqliteConnection) -> QueryResult<Vec<Rustacean>> {
        rustaceans::table
            .order(rustaceans::id.desc())
            .limit(1000)
            .load::<Rustacean>(c)
    }

    pub fn create_rustacean(c: &mut SqliteConnection, rustacean: RustaceanData) -> QueryResult<Rustacean> {
        diesel::insert_into(rustaceans::table).values(rustacean).execute(c)?;
        let last_id = Self::last_inserted_id(c)?;
        Self::get_rustacean(c, last_id)
    }

    pub fn update_rastacean(c: &mut SqliteConnection, id: i32, rustacean: RustaceanData) -> QueryResult<Rustacean> {
        diesel::update(rustaceans::table.find(id)).set(rustacean).execute(c)?;
        Self::get_rustacean(c, id)
    }

    pub fn delete_rustacean(c: &mut SqliteConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(rustaceans::table.find(id)).execute(c)
    }
}
