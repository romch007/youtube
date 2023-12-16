// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "tsvector", schema = "pg_catalog"))]
    pub struct Tsvector;
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Tsvector;

    videos (id) {
        id -> Int4,
        title -> Varchar,
        description -> Text,
        bucket -> Varchar,
        published_at -> Timestamptz,
        author_id -> Int4,
    }
}

diesel::joinable!(videos -> users (author_id));

diesel::allow_tables_to_appear_in_same_query!(users, videos,);
