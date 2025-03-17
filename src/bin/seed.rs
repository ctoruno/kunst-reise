use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenvy::dotenv;
use std::env;
use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;
use std::path::Path;

// Import your models and schema
use kunstreise::models::{School, Artist, Painting};
use kunstreise::schema::{schools, artists, paintings};

fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|e| panic!("Error connecting to {}: {}", database_url, e))
}

fn seed_schools(conn: &mut SqliteConnection) -> Result<(), Box<dyn Error>> {
    let csv_path = Path::new("data/schools.csv");
    let file = File::open(csv_path)?;
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

    for result in rdr.deserialize() {
        let record: School = result?;
        diesel::insert_into(schools::table)
            .values(&record)
            .execute(conn)?;
    }

    println!("✅ Schools data seeded successfully");
    Ok(())
}

fn seed_artists(conn: &mut SqliteConnection) -> Result<(), Box<dyn Error>> {
    let csv_path = Path::new("data/artists.csv");
    let file = File::open(csv_path)?;
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

    for result in rdr.deserialize() {
        let record: Artist = result?;
        diesel::insert_into(artists::table)
            .values(&record)
            .execute(conn)?;
    }

    println!("✅ Artists data seeded successfully");
    Ok(())
}

fn seed_paintings(conn: &mut SqliteConnection) -> Result<(), Box<dyn Error>> {
    let csv_path = Path::new("data/paintings_deduplicated.csv");
    let file = File::open(csv_path)?;
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

    for result in rdr.deserialize() {
        let record: Painting = result?;
        diesel::insert_into(paintings::table)
            .values(&record)
            .execute(conn)?;
    }

    println!("✅ Paintings data seeded successfully");
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut conn = establish_connection();

    // Clear existing data (optional)
    diesel::delete(paintings::table).execute(&mut conn)?;
    diesel::delete(artists::table).execute(&mut conn)?;
    diesel::delete(schools::table).execute(&mut conn)?;

    // Seed tables in order of dependencies
    seed_schools(&mut conn)?;
    seed_artists(&mut conn)?;
    seed_paintings(&mut conn)?;

    println!("🎉 Database seeded successfully!");
    Ok(())
}