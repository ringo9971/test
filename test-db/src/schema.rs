// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 128]
        user_uid -> Varchar,
        created_at -> Timestamp,
        #[max_length = 128]
        name -> Varchar,
        #[max_length = 255]
        email -> Nullable<Varchar>,
    }
}
