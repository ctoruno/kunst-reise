-- Your SQL goes here
CREATE TABLE schools (
    movement VARCHAR(50) PRIMARY KEY NOT NULL,
    time_period VARCHAR(50) NOT NULL,
    context TEXT NOT NULL,
    characteristics TEXT NOT NULL,
    slug VARCHAR(50) NOT NULL
);

CREATE TABLE artists (
    artist_name VARCHAR(50) PRIMARY KEY NOT NULL, 
    country VARCHAR(50) NOT NULL,
    time_period VARCHAR(50) NOT NULL,
    movement VARCHAR(50) REFERENCES schools(movement) NOT NULL,
    biography TEXT NOT NULL,
    characteristics TEXT NOT NULL,
    slug VARCHAR(50) NOT NULL
);

CREATE TABLE paintings (
    artist_name VARCHAR(50) REFERENCES artists(artist_name) NOT NULL,
    painting_name VARCHAR(50) PRIMARY KEY NOT NULL,
    year VARCHAR(50) NOT NULL,
    context TEXT NOT NULL,
    location TEXT NOT NULL,
    slug VARCHAR(50) NOT NULL
)