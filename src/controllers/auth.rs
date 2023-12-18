use crate::models::UserWithVideos;
use crate::{auth, config, errors, models, schema, AppState};

use argon2::{PasswordHash, PasswordVerifier};

use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{extract::State, Json};
use axum::{Extension, Router};

use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use serde::Deserialize;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};

pub fn router<S>(state: AppState) -> Router<S> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route(
            "/me",
            get(me).route_layer(axum::middleware::from_fn_with_state(
                state.clone(),
                auth::middleware,
            )),
        )
        .with_state(state)
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = schema::users)]
pub struct UserInfo {
    username: String,
    password: String,
}

pub async fn register(
    State(state): State<AppState>,
    Json(mut user): Json<UserInfo>,
) -> Result<Json<models::User>, (StatusCode, String)> {
    use schema::users::dsl::users;

    let mut conn = state.db_pool.get().await.map_err(errors::internal_error)?;

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(user.password.as_bytes(), &salt)
        .map_err(errors::internal_error)?
        .to_string();

    user.password = password_hash;

    let created_user = diesel::insert_into(users)
        .values(user)
        .get_result(&mut conn)
        .await
        .map_err(errors::internal_error)?;

    Ok(Json(created_user))
}

pub async fn login(
    State(state): State<AppState>,
    Json(login_info): Json<UserInfo>,
) -> Result<String, (StatusCode, String)> {
    use schema::users::dsl::{username, users};

    let mut conn = state.db_pool.get().await.map_err(errors::internal_error)?;

    let target_user = users
        .filter(username.eq(login_info.username))
        .select(models::User::as_select())
        .first(&mut conn)
        .await
        .optional()
        .map_err(errors::internal_error)?
        .ok_or_else(|| (StatusCode::FORBIDDEN, "Forbidden".to_string()))?;

    let password_hash = PasswordHash::new(&target_user.password).map_err(errors::internal_error)?;

    let matching_passwords = Argon2::default()
        .verify_password(login_info.password.as_bytes(), &password_hash)
        .is_ok();

    if !matching_passwords {
        return Err((StatusCode::FORBIDDEN, "Forbidden".to_string()));
    }

    let exp_date = chrono::Utc::now() + chrono::Duration::days(7);

    let jwt_claims = auth::JwtClaims {
        user_id: target_user.id,
        exp: exp_date.timestamp() as usize,
    };

    let jwt_secret = config::config().await.jwt_secret();

    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &jwt_claims,
        &jsonwebtoken::EncodingKey::from_secret(jwt_secret.as_ref()),
    )
    .map_err(errors::internal_error)?;

    Ok(token)
}

pub async fn me(
    State(state): State<AppState>,
    Extension(user): Extension<models::User>,
) -> Result<Json<models::UserWithVideos>, (StatusCode, String)> {
    let mut conn = state.db_pool.get().await.map_err(errors::internal_error)?;

    let related_videos = models::Video::belonging_to(&user)
        .select(models::Video::as_select())
        .load(&mut conn)
        .await
        .map_err(errors::internal_error)?;

    let user_with_videos = UserWithVideos {
        user,
        videos: related_videos,
    };

    Ok(Json(user_with_videos))
}
