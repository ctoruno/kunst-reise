use diesel::prelude::*;
use diesel::result::QueryResult;
use dotenvy::dotenv;
use std::env;
use crate::models::{School, Artist, Painting};
use crate::schema::{schools, artists, paintings};


pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}


pub fn get_all_schools(conn: &mut SqliteConnection) -> QueryResult<Vec<School>> {
    schools::table.load::<School>(conn)
}


pub fn get_all_artists(conn: &mut SqliteConnection) -> QueryResult<Vec<Artist>> {
    artists::table.load::<Artist>(conn)
}


pub fn get_school_data(conn: &mut SqliteConnection, movement_slug: &str) -> QueryResult<School> {
    schools::table
        .filter(schools::slug.eq(movement_slug))
        .first::<School>(conn)
}


pub fn get_artist_data(conn: &mut SqliteConnection, artist_slug: &str) -> QueryResult<Artist> {
    artists::table
        .filter(artists::slug.eq(artist_slug))
        .first::<Artist>(conn)
}


pub fn get_artists_by_movement(conn: &mut SqliteConnection, movement_name: &str) -> QueryResult<Vec<Artist>> {
    artists::table
        .filter(artists::movement.eq(movement_name))
        .load::<Artist>(conn)
}


pub fn get_paintings_by_artists(
    conn: &mut SqliteConnection, 
    artist_names: &[String],
    limit: Option<i64>
) -> QueryResult<Vec<Painting>> {
    let mut query = paintings::table
        .filter(paintings::artist_name.eq_any(artist_names))
        .order_by(diesel::dsl::sql::<diesel::sql_types::Text>("RANDOM()"))
        .into_boxed();
    
    if let Some(limit_value) = limit {
        query = query.limit(limit_value);
    }
    
    query.load::<Painting>(conn)
}