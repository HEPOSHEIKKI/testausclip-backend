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
        ownerid -> Nullable<Varchar>,
        game -> Nullable<Text>,
        uploaddate -> Nullable<Timestamp>,
        #[max_length = 255]
        filename -> Nullable<Varchar>,
    }
}

diesel::table! {
    likes (userid, clipid) {
        #[max_length = 255]
        userid -> Varchar,
        #[max_length = 255]
        clipid -> Varchar,
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
diesel::joinable!(likes -> clips (clipid));
diesel::joinable!(likes -> users (userid));

diesel::allow_tables_to_appear_in_same_query!(
    clips,
    likes,
    users,
);
