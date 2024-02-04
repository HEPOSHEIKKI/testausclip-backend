// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Text,
        title -> Nullable<Text>,
        description -> Nullable<Text>,
        private -> Nullable<Bool>,
        ownerid -> Nullable<Int4>,
    }
}

diesel::table! {
    users (userid) {
        userid -> Int4,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        password -> Varchar,
    }
}

diesel::joinable!(posts -> users (ownerid));

diesel::allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
