// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 50]
        username -> Varchar,
        #[max_length = 100]
        email -> Nullable<Varchar>,
        password -> Bytea,
        salt -> Bytea,
        registration_date -> Nullable<Timestamp>,
    }
}
