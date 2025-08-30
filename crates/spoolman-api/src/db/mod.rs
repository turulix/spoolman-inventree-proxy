mod update_pending;

use sqlx::{Connection, SqlitePool};

#[derive(Clone)]
pub struct DbClient {
    db: SqlitePool,
}

impl DbClient {
    pub async fn new(file_path: &str) -> Self {
        Self {
            db: SqlitePool::connect(file_path).await.unwrap(),
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
