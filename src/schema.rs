// @generated automatically by Diesel CLI.

diesel::table! {
    clips (id) {
        id -> Text,
        #[max_length = 100]
        title -> Nullable<Varchar>,
        description -> Nullable<Text>,
        private -> Nullable<Bool>,
        #[max_length = 255]
        ownerid -> Nullable<Varchar>,
        game -> Nullable<Text>,
        uploaddate -> Nullable<Timestamp>,
        #[max_length = 255]
        filename -> Nullable<Varchar>,
    }
}

diesel::table! {
    users (userid) {
        #[max_length = 255]
        userid -> Varchar,
        #[max_length = 50]
        username -> Varchar,
        #[max_length = 100]
        email -> Nullable<Varchar>,
        #[max_length = 100]
        passwordhash -> Varchar,
        registrationdate -> Nullable<Timestamp>,
    }
}

diesel::joinable!(clips -> users (ownerid));

diesel::allow_tables_to_appear_in_same_query!(
    clips,
    users,
);
