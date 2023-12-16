use crate::{auth, errors, models, schema, AppState};

use errors::NotFoundExt;

use axum::extract::{DefaultBodyLimit, Path};
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{extract::State, Json};
use axum::{Extension, Router};

use axum_typed_multipart::{FieldData, TryFromMultipart, TypedMultipart};

use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use serde::Deserialize;
use tempfile::NamedTempFile;

pub fn router<S>(state: AppState) -> Router<S> {
    Router::new()
        .route("/", get(list_videos))
        .route(
            "/upload",
            post(upload).route_layer(axum::middleware::from_fn_with_state(
                state.clone(),
                auth::middleware,
            )),
        )
        .route("/:id", get(get_video))
        .layer(DefaultBodyLimit::disable())
        .with_state(state)
}

#[derive(Debug, Deserialize)]
struct ListVideoQuery {
    search: Option<String>,
}

async fn list_videos(
    State(state): State<AppState>,
    axum::extract::Query(query): axum::extract::Query<ListVideoQuery>,
) -> Result<Json<Vec<models::VideoWithAuthor>>, (StatusCode, String)> {
    use schema::users::dsl::users;
    use schema::videos::dsl::title;
    use schema::videos::dsl::videos;

    let mut conn = state.db_pool.get().await.unwrap();

    let mut sql_query = videos.inner_join(users).into_boxed();

    if let Some(search_term) = query.search {
        if !search_term.is_empty() {
            let pattern = format!("%{search_term}%");
            sql_query = sql_query.filter(title.like(pattern));
        }
    }

    let res = sql_query
        .select((models::Video::as_select(), models::User::as_select()))
        .load::<(models::Video, models::User)>(&mut conn)
        .await
        .map_err(errors::internal_error)?;

    let videos_with_author = res
        .into_iter()
        .map(|(video, author)| models::VideoWithAuthor { video, author })
        .collect::<Vec<_>>();

    Ok(Json(videos_with_author))
}

#[derive(TryFromMultipart)]
struct UploadVideoRequest {
    title: String,
    description: String,
    #[form_data(limit = "unlimited")]
    video: FieldData<NamedTempFile>,
}

async fn upload(
    State(state): State<AppState>,
    Extension(logged_user): Extension<models::User>,
    TypedMultipart(upload_request): TypedMultipart<UploadVideoRequest>,
) -> Result<Json<models::Video>, (StatusCode, String)> {
    use schema::videos::dsl::videos;

    let bucket_id = uuid::Uuid::new_v4().to_string();

    let mut tmp_video_file = tokio::fs::File::from_std(upload_request.video.contents.into_file());

    state
        .s3
        .put_object_stream(&mut tmp_video_file, &bucket_id)
        .await
        .map_err(errors::internal_error)?;

    let mut conn = state.db_pool.get().await.unwrap();

    let new_video = models::NewVideo {
        title: upload_request.title,
        description: upload_request.description,
        bucket: bucket_id,
        author_id: logged_user.id,
    };

    let inserted_video = diesel::insert_into(videos)
        .values(&new_video)
        .get_result(&mut conn)
        .await
        .map_err(errors::internal_error)?;

    Ok(Json(inserted_video))
}

async fn get_video(
    State(state): State<AppState>,
    Path(video_id): Path<i32>,
) -> Result<Json<models::Video>, (StatusCode, String)> {
    use schema::videos::dsl::videos;

    let mut conn = state.db_pool.get().await.unwrap();

    let target_video = videos
        .select(models::Video::as_select())
        .find(video_id)
        .first(&mut conn)
        .await
        .optional()
        .map_err(errors::internal_error)?
        .map_not_found()?;

    Ok(Json(target_video))
}
