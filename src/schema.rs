// @generated automatically by Diesel CLI.

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::*;

    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::*;

    videos (id) {
        id -> Int4,
        title -> Varchar,
        description -> Text,
        duration_seconds -> Int8,
        bucket -> Uuid,
        published_at -> Timestamptz,
        author_id -> Int4,
        textsearchable_index_col -> Tsvector,
    }
}

diesel::joinable!(videos -> users (author_id));

diesel::allow_tables_to_appear_in_same_query!(
    users,
    videos,
);
