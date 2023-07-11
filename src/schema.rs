// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int8,
        name -> Text,
        year_of_birth -> Int8,
    }
}
