use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Deserialize, Serialize, Insertable)]
#[diesel(table_name = crate::schema::schools)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct School {
    pub movement: String,
    pub time_period: String,
    pub context: String,
    pub characteristics: String,
    pub slug: String,
}

#[derive(Queryable, Selectable, Deserialize, Serialize, Insertable)]
#[diesel(table_name = crate::schema::artists)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Artist {
    pub artist_name: String,
    pub country: String,
    pub time_period: String,
    pub movement: String,
    pub biography: String,
    pub characteristics: String,
    pub slug: String,
}

#[derive(Queryable, Selectable, Deserialize, Serialize, Insertable)]
#[diesel(table_name = crate::schema::paintings)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Painting {
    pub artist_name: String,
    pub painting_name: String,
    pub year: String,
    pub context: String,
    pub location: String,
    pub slug: String,
}