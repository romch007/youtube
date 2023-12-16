use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Queryable, Selectable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: String,

    #[serde(skip_serializing)]
    pub password: String,
}

#[derive(Debug, Queryable, Selectable, Identifiable, Associations, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::videos)]
#[diesel(belongs_to(User, foreign_key = author_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Video {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub bucket: String,

    #[serde(skip_deserializing)]
    pub published_at: chrono::DateTime<chrono::Utc>,

    pub author_id: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::schema::videos)]
pub struct NewVideo {
    pub title: String,
    pub description: String,
    pub bucket: String,
    pub author_id: i32,
}

#[derive(Debug, Serialize)]
pub struct UserWithVideos {
    #[serde(flatten)]
    pub user: User,
    pub videos: Vec<Video>,
}

#[derive(Debug, Serialize)]
pub struct VideoWithAuthor {
    #[serde(flatten)]
    pub video: Video,
    pub author: User,
}