mod auth;
mod config;
mod controllers;
mod db;
mod errors;
mod models;
mod schema;

use std::net::SocketAddr;

use axum::{extract::State, http::StatusCode, routing::get, Router};

use diesel_async::pooled_connection::AsyncDieselConnectionManager;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Debug, Clone)]
pub struct AppState {
    pub db_pool: db::Pool,
    pub s3: s3::Bucket,
}

#[tokio::main]
async fn main() {
    // Setup logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app_state = AppState {
        db_pool: create_database_pool().await,
        s3: create_s3_bucket().await,
    };

    let app = Router::new()
        .route("/health", get(health))
        .merge(controllers::auth::router(app_state.clone()))
        .nest("/videos", controllers::videos::router(app_state.clone()))
        .with_state(app_state);

    let config = config::config().await;

    let addr = format!("{}:{}", config.server_host(), config.server_port());
    let addr: SocketAddr = addr.parse().expect("invalid socket address");
    tracing::info!("listening on {addr}");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn create_s3_bucket() -> s3::Bucket {
    let config = config::config().await;

    let region = s3::Region::Custom {
        region: "eu-central-1".to_owned(),
        endpoint: config.s3_base_url().to_owned(),
    };

    let credentials = s3::creds::Credentials::new(
        Some(config.s3_access_key()),
        Some(config.s3_secret_key()),
        None,
        None,
        None,
    )
    .expect("invalid S3 credentials");

    let mut bucket = s3::Bucket::new(config.s3_bucket(), region.clone(), credentials.clone())
        .expect("cannot access S3 bucket")
        .with_path_style();

    if !bucket
        .exists()
        .await
        .expect("cannot check if bucket exists")
    {
        tracing::info!(
            "bucket {} does not exist, creating it...",
            config.s3_bucket()
        );

        bucket = s3::Bucket::create_with_path_style(
            config.s3_bucket(),
            region,
            credentials,
            s3::BucketConfiguration::default(),
        )
        .await
        .expect("cannot create bucket")
        .bucket;
    }

    bucket
}

async fn create_database_pool() -> db::Pool {
    let config = config::config().await;

    let database_config =
        AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(config.db_url());

    bb8::Pool::builder()
        .connection_timeout(std::time::Duration::from_secs(5))
        .build(database_config)
        .await
        .expect("cannot create database pool")
}

async fn health(State(state): State<AppState>) -> Result<&'static str, (StatusCode, String)> {
    let mut conn = state.db_pool.get().await.map_err(errors::internal_error)?;

    use diesel_async::RunQueryDsl;

    diesel::sql_query("select 1")
        .execute(&mut conn)
        .await
        .map_err(errors::internal_error)?;

    Ok("I'm alive!")
}
