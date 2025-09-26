// @generated automatically by Diesel CLI.

pub mod fittrack {
    diesel::table! {
        fittrack.users (username) {
            id -> Int4,
            #[max_length = 50]
            username -> Varchar,
            #[max_length = 100]
            fullname -> Varchar,
            #[max_length = 100]
            email -> Varchar,
            #[max_length = 255]
            password -> Varchar,
            created_at -> Nullable<Timestamp>,
        }
    }
}
