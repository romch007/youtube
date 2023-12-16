use crate::{config, errors, models, schema, AppState};
use axum::{
    extract::{Request, State},
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
};
use serde::{Deserialize, Serialize};

use diesel::prelude::*;
use diesel_async::RunQueryDsl;

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    pub user_id: i32,
    pub exp: usize,
}

pub async fn middleware(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response, (StatusCode, String)> {
    let token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|auth_header| auth_header.to_str().ok())
        .and_then(|auth_value| auth_value.strip_prefix("Bearer "));

    let token = token.ok_or((
        StatusCode::UNAUTHORIZED,
        "You are not logged in".to_string(),
    ))?;

    let jwt_secret = config::config().await.jwt_secret();

    let claims = jsonwebtoken::decode::<JwtClaims>(
        token,
        &jsonwebtoken::DecodingKey::from_secret(jwt_secret.as_ref()),
        &jsonwebtoken::Validation::default(),
    )
    .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid token".to_string()))?
    .claims;

    use schema::users::dsl::users;

    let mut conn = state.db_pool.get().await.map_err(errors::internal_error)?;

    let logged_user = users
        .select(models::User::as_select())
        .find(claims.user_id)
        .first(&mut conn)
        .await
        .optional()
        .map_err(errors::internal_error)?
        .ok_or((StatusCode::UNAUTHORIZED, "Invalid token".to_string()))?;

    req.extensions_mut().insert(logged_user);

    Ok(next.run(req).await)
}
