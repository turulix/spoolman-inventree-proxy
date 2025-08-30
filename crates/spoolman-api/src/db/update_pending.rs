use crate::db::DbClient;
use inventree::stock::StockItemId;
use sqlx::{SqliteConnection, query};

impl DbClient {
    pub async fn update_pending_spool_usage(
        &self,
        conn: &mut SqliteConnection,
        spool_id: StockItemId,
        pending_weight: f64,
    ) -> Result<(), anyhow::Error> {
        let cast_spool_id: i64 = spool_id.0 as i64;
        let now = chrono::Utc::now();
        query!(
            r#"
            INSERT INTO pending_spool_usage (spool_id, pending_weight, last_updated_at)
            VALUES (?, ?, ?)
            ON CONFLICT DO UPDATE SET pending_weight  = pending_weight + EXCLUDED.pending_weight,
                          last_updated_at = EXCLUDED.last_updated_at;
        "#,
            cast_spool_id,
            pending_weight,
            now
        )
        .execute(conn)
        .await?;
        Ok(())
    }

    pub async fn select_pending_spool_usage(
        &self,
        conn: &mut SqliteConnection,
        spool_id: StockItemId,
    ) -> Result<Option<f64>, anyhow::Error> {
        let cast_spool_id: i64 = spool_id.0 as i64;
        let record = query!(
            r#"
            SELECT pending_weight FROM pending_spool_usage WHERE spool_id = ?;
        "#,
            cast_spool_id
        )
        .fetch_optional(conn)
        .await?;
        Ok(record.map(|r| r.pending_weight))
    }
}
