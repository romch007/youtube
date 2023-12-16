use axum::http::StatusCode;

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
pub fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

pub trait NotFoundExt<T> {
    fn map_not_found(self) -> Result<T, (StatusCode, String)>;
}

impl<T> NotFoundExt<T> for Option<T> {
    fn map_not_found(self) -> Result<T, (StatusCode, String)> {
        match self {
            Some(v) => Ok(v),
            None => Err((StatusCode::NOT_FOUND, "Not Found".to_string())),
        }
    }
}
