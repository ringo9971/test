// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        timestamp -> Timestamp,
        #[max_length = 128]
        name -> Varchar,
    }
}
