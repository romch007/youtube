use crate::{auth, errors, models, schema, video_util, AppState};

use errors::NotFoundExt;

use axum::extract::{DefaultBodyLimit, Path};
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{extract::State, Json};
use axum::{Extension, Router};

use axum_typed_multipart::{FieldData, TryFromMultipart, TypedMultipart};

use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use diesel_full_text_search::*;

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
    axum::extract::Query(params): axum::extract::Query<ListVideoQuery>,
) -> Result<Json<Vec<models::VideoWithAuthor>>, (StatusCode, String)> {
    use schema::users::dsl::users;
    use schema::videos::dsl::textsearchable_index_col;
    use schema::videos::dsl::videos;

    let selection = (models::VIDEO_ALL_COLUMNS, models::User::as_select());

    let mut query = videos.inner_join(users).select(selection).into_boxed();

    if let Some(search_term) = params.search {
        if !search_term.is_empty() {
            let q = diesel::dsl::sql::<TsQuery>("plainto_tsquery('english', ")
                .bind::<diesel::sql_types::Text, _>(search_term)
                .sql(")");

            query = query.filter(q.clone().matches(textsearchable_index_col));

            let rank = ts_rank_cd(textsearchable_index_col, q);
            query = query.then_order_by(rank.desc())
        }
    }

    let mut conn = state.db_pool.get().await.map_err(errors::internal_error)?;

    let res = query
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

    // FIXME: don't return internal server error if the uploaded file is not a video
    let video_duration = video_util::get_video_duration(upload_request.video.contents.path())
        .map_err(errors::internal_error)?;

    let mut tmp_video_file = tokio::fs::File::from_std(upload_request.video.contents.into_file());

    state
        .s3
        .put_object_stream(&mut tmp_video_file, &bucket_id)
        .await
        .map_err(errors::internal_error)?;

    let mut conn = state.db_pool.get().await.map_err(errors::internal_error)?;

    let new_video = models::NewVideo {
        title: upload_request.title,
        description: upload_request.description,
        duration_seconds: video_duration.num_seconds(),
        bucket: bucket_id,
        author_id: logged_user.id,
    };

    let inserted_video = diesel::insert_into(videos)
        .values(&new_video)
        .returning(models::VIDEO_ALL_COLUMNS)
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

    let mut conn = state.db_pool.get().await.map_err(errors::internal_error)?;

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
