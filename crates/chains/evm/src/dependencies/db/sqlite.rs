use anyhow::Result;
use sqlx::SqlitePool;
use std::{env, fs, path::Path};
use tracing::{debug, error, info};

pub struct SqliteStorage {
    pool: SqlitePool,
}

impl SqliteStorage {
    #[tracing::instrument(
        name = "sqlite_init",
        skip_all,
        fields(network = %network)
    )]
    pub async fn init(network: &str) -> Result<Self> {
        // Get base directory from env or default
        let base = env::var("SQLITE_DB_DIR").unwrap_or_else(|_| "./data".into());

        // Canonicalize to absolute path
        let base_abs = fs::canonicalize(&base).unwrap_or_else(|_| Path::new(&base).to_path_buf());

        // Ensure directory exists
        fs::create_dir_all(&base_abs)?;

        // Build full path to db file
        let path = base_abs.join(format!("{network}.db"));

        // Pre‑create file if missing
        if !path.exists() {
            fs::File::create(&path)?;
        }

        let url = format!("sqlite://{}", path.to_string_lossy());

        info!(db_path = %path.display(), "Initializing SQLite storage");

        let pool = SqlitePool::connect(&url).await.map_err(|e| {
            error!(error = %e, db_url = %url, "Failed to connect to SQLite");
            e
        })?;

        debug!("Ensuring SQLite tables exist");

        // processed_conversions
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS processed_conversions (
                tx_id INTEGER PRIMARY KEY,
                processed INTEGER NOT NULL,
                btc_tx_id TEXT
            )",
        )
        .execute(&pool)
        .await?;

        // receive_state
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS receive_state (
                tx_id TEXT PRIMARY KEY,
                idx INTEGER NOT NULL
            )",
        )
        .execute(&pool)
        .await?;

        info!("SQLite schema ensured");

        Ok(Self { pool })
    }

    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }
}
