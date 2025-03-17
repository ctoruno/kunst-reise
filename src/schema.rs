// @generated automatically by Diesel CLI.

diesel::table! {
    artists (artist_name) {
        artist_name -> Text,
        country -> Text,
        time_period -> Text,
        movement -> Text,
        biography -> Text,
        characteristics -> Text,
        slug -> Text,
    }
}

diesel::table! {
    paintings (painting_name) {
        artist_name -> Text,
        painting_name -> Text,
        year -> Text,
        context -> Text,
        location -> Text,
        slug -> Text,
    }
}

diesel::table! {
    schools (movement) {
        movement -> Text,
        time_period -> Text,
        context -> Text,
        characteristics -> Text,
        slug -> Text,
    }
}

diesel::joinable!(artists -> schools (movement));
diesel::joinable!(paintings -> artists (artist_name));

diesel::allow_tables_to_appear_in_same_query!(
    artists,
    paintings,
    schools,
);
