use crate::schema::*;
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
    pub duration_seconds: i64,
    pub bucket: uuid::Uuid,

    #[serde(skip_deserializing)]
    pub published_at: chrono::DateTime<chrono::Utc>,

    pub author_id: i32,
}

/// We literally never want to select `textsearchable_index_col`
/// so we provide this type and constant to pass to `.select`
type VideoAllColumns = (
    videos::id,
    videos::title,
    videos::description,
    videos::duration_seconds,
    videos::bucket,
    videos::published_at,
    videos::author_id,
);

pub const VIDEO_ALL_COLUMNS: VideoAllColumns = (
    videos::id,
    videos::title,
    videos::description,
    videos::duration_seconds,
    videos::bucket,
    videos::published_at,
    videos::author_id,
);

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::schema::videos)]
pub struct NewVideo {
    pub title: String,
    pub description: String,
    pub bucket: uuid::Uuid,
    pub duration_seconds: i64,
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

#[derive(Identifiable, Selectable, Queryable, Associations, Debug)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Video))]
#[diesel(table_name = likes)]
#[diesel(primary_key(user_id, video_id))]
pub struct Like {
    pub user_id: i32,
    pub video_id: i32,
    pub is_liking: bool,
}
