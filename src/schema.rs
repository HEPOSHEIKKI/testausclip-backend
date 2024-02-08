// @generated automatically by Diesel CLI.

diesel::table! {
    clips (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 100]
        title -> Nullable<Varchar>,
        description -> Nullable<Text>,
        private -> Nullable<Bool>,
        #[max_length = 255]
        owner_id -> Nullable<Varchar>,
        game -> Nullable<Text>,
        upload_date -> Nullable<Timestamp>,
        #[max_length = 255]
        file_name -> Nullable<Varchar>,
        views -> Nullable<Int4>,
    }
}

diesel::table! {
    likes (user_id, clip_id) {
        #[max_length = 255]
        user_id -> Varchar,
        #[max_length = 255]
        clip_id -> Varchar,
    }
}

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
        #[max_length = 32]
        auth_token -> Varchar,
        registration_date -> Nullable<Timestamp>,
    }
}

diesel::joinable!(clips -> users (owner_id));
diesel::joinable!(likes -> clips (clip_id));
diesel::joinable!(likes -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    clips,
    likes,
    users,
);
