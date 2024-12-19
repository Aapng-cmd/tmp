// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "access_type"))]
    pub struct AccessType;
}

diesel::table! {
    comments (id) {
        id -> Int4,
        text -> Text,
        from_id -> Int4,
        post_id -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::AccessType;

    post_accesses (id) {
        id -> Int4,
        user_id -> Int4,
        post_id -> Int4,
        access_type -> AccessType,
        created_at -> Timestamp,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        #[max_length = 255]
        secret_key -> Nullable<Varchar>,
        #[max_length = 255]
        filename -> Varchar,
        user_id -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 50]
        username -> Varchar,
        #[max_length = 64]
        password -> Varchar,
    }
}

diesel::joinable!(comments -> posts (post_id));
diesel::joinable!(comments -> users (from_id));
diesel::joinable!(post_accesses -> posts (post_id));
diesel::joinable!(post_accesses -> users (user_id));
diesel::joinable!(posts -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    comments,
    post_accesses,
    posts,
    users,
);
