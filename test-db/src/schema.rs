// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        created_at -> Timestamp,
        #[max_length = 128]
        name -> Varchar,
    }
}
