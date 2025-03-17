#[macro_use] extern crate rocket;

use rocket_dyn_templates::{Template, context};
use slug::slugify;
use std::collections::HashMap;
use kunstreise::db::{establish_connection, get_artist_data};
use kunstreise::models::{School, Artist, Painting};
use kunstreise::db::{
    get_all_schools, get_school_data, get_artists_by_movement,
    get_paintings_by_artists, get_all_artists
};


#[get("/")]
fn index() -> Template {
    let context: HashMap<String, String> = HashMap::new();
    Template::render("index", &context)
}


#[get("/escuelas")]
fn escuelas() -> Template {

    let connection = &mut establish_connection();

    let art_movements: Vec<School> = get_all_schools(connection)
        .expect("Error loading schools");

    let mut content = HashMap::new();
    content.insert("art_movements", art_movements);

    Template::render("escuelas", &content)
}


#[get("/artists")]
fn artists() -> Template {

    let connection = &mut establish_connection();

    let art_movements: Vec<School> = get_all_schools(connection)
        .expect("Error loading schools");

    let artists_data: Vec<Artist> = get_all_artists(connection)
        .expect("Error loading artists");
    
    Template::render("artists", context! {
        art_movements: art_movements,
        artists: artists_data,
    })
}


#[get("/escuelas/<movement_slug>")]
fn escuela_layout(movement_slug: String) -> Template {

    let connection = &mut establish_connection();

    let movement_data = get_school_data(connection, &movement_slug)
        .expect("Could not find the movement");

    let artists_data = get_artists_by_movement(connection, &movement_data.movement)
        .expect("Could not load artists");

    let artist_names: Vec<String> = artists_data
        .iter()
        .map(|artist| artist.artist_name.clone())
        .collect();

    let paintings_data = get_paintings_by_artists(connection, &artist_names, Some(4))
        .expect("Could not load paintings");

    Template::render("escuela-layout", context! {
        movement: movement_data,
        artists: artists_data,
        paintings: paintings_data,
    })
}


#[get("/artists/<artist_slug>")]
fn artist_layout(artist_slug: String) -> Template{
    
    let connection = &mut establish_connection();

    let artist_data = get_artist_data(connection, &artist_slug)
        .expect("Could not find artist data");

    let paintings_data = get_paintings_by_artists(connection, &[artist_data.artist_name.clone()], Some(5))
        .expect("Could not load paintings");

    let movement_slug = slugify(&artist_data.movement);

    Template::render("artist-layout", context! {
        artist_data: artist_data,
        paintings: paintings_data,
        movement_slug: movement_slug,
    })
}


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![
            index, escuelas, escuela_layout, artists, artist_layout
        ])
        .mount("/static", rocket::fs::FileServer::from("static"))
        .attach(Template::fairing())
}
