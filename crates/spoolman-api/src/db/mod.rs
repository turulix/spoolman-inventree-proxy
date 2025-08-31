mod update_pending;

use sqlx::SqlitePool;
use sqlx::sqlite::{SqliteAutoVacuum, SqliteConnectOptions};
use std::str::FromStr;
use tokio::fs::create_dir_all;

#[derive(Clone)]
pub struct DbClient {
    db: SqlitePool,
}

impl DbClient {
    pub async fn new(db_url: &str) -> Self {
        // Ensure the data directory exists
        create_dir_all("data").await.unwrap();

        let options = SqliteConnectOptions::from_str(db_url)
            .unwrap()
            .auto_vacuum(SqliteAutoVacuum::Incremental)
            .create_if_missing(true);

        Self {
            db: SqlitePool::connect_with(options).await.unwrap(),
        }
    }

    #[inline(always)]
    pub async fn begin(&self) -> Result<sqlx::Transaction<'_, sqlx::Sqlite>, sqlx::Error> {
        self.db.begin().await
    }

    #[inline(always)]
    pub async fn acquire(&self) -> Result<sqlx::pool::PoolConnection<sqlx::Sqlite>, sqlx::Error> {
        self.db.acquire().await
    }
}
