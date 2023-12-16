use dotenvy::dotenv;
use std::env;
use tokio::sync::OnceCell;

#[derive(Debug)]
struct ServerConfig {
    host: String,
    port: u16,
}

#[derive(Debug)]
struct DatabaseConfig {
    url: String,
}

#[derive(Debug)]
struct S3Config {
    base_url: String,
    access_key: String,
    secret_key: String,
    bucket: String,
}

#[derive(Debug)]
pub struct Config {
    server: ServerConfig,
    db: DatabaseConfig,
    s3: S3Config,
    jwt_secret: String,
}

impl Config {
    pub fn db_url(&self) -> &str {
        &self.db.url
    }

    pub fn server_host(&self) -> &str {
        &self.server.host
    }

    pub fn server_port(&self) -> u16 {
        self.server.port
    }

    pub fn jwt_secret(&self) -> &str {
        &self.jwt_secret
    }

    pub fn s3_base_url(&self) -> &str {
        &self.s3.base_url
    }

    pub fn s3_access_key(&self) -> &str {
        &self.s3.access_key
    }

    pub fn s3_secret_key(&self) -> &str {
        &self.s3.secret_key
    }

    pub fn s3_bucket(&self) -> &str {
        &self.s3.bucket
    }
}

pub static CONFIG: OnceCell<Config> = OnceCell::const_new();

async fn init_config() -> Config {
    dotenv().ok();

    let server_config = ServerConfig {
        host: env::var("HOST").unwrap_or_else(|_| String::from("127.0.0.1")),
        port: env::var("PORT")
            .unwrap_or_else(|_| String::from("3000"))
            .parse::<u16>()
            .unwrap(),
    };

    let database_config = DatabaseConfig {
        url: require_env("DATABASE_URL"),
    };

    let s3_config = S3Config {
        base_url: require_env("S3_BASE_URL"),
        access_key: require_env("S3_ACCESS_KEY"),
        secret_key: require_env("S3_SECRET_KEY"),
        bucket: require_env("S3_BUCKET"),
    };

    let jwt_secret = require_env("JWT_SECRET");

    Config {
        server: server_config,
        db: database_config,
        s3: s3_config,
        jwt_secret,
    }
}

pub async fn config() -> &'static Config {
    CONFIG.get_or_init(init_config).await
}

fn require_env(var_name: &str) -> String {
    env::var(var_name).unwrap_or_else(|_| panic!("{var_name} is not set"))
}
