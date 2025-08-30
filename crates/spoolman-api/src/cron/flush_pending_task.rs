use crate::context::Context;
use chrono::Utc;
use inventree::stock::StockItemId;
use log::{debug, error};
use std::time::Duration;
use tokio::task;

pub fn start_flushing_job(ctx: Context) {
    // We create a new runtime to run the job on.
    // This is optional if your application already has a global runtime.
    task::spawn(async move {
        // Set the interval for the job to run (e.g., every 1 minutes).
        let mut interval = tokio::time::interval(Duration::from_secs(60));

        debug!("Starting flushing job");

        // The job runs indefinitely.
        loop {
            // Wait for the next tick of the interval.
            interval.tick().await;

            debug!("Flushing job running...");

            // Call the function that performs the flushing logic.
            // We use 'clone()' here because the context is moved into the task.
            if let Err(e) = flush_spool_usage_to_inventree(&ctx).await {
                // Log any errors that occur during the flush operation.
                error!("Error flushing spool usage: {}", e);
            }
        }
    });
}

async fn flush_spool_usage_to_inventree(ctx: &Context) -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = ctx.db.acquire().await?;
    // 1. Query the database for pending spool usage.
    // This is where you would execute your SQL query to get all entries
    // from the `pending_spool_usage` table where `pending_weight` > 0.
    // Example using sqlx:
    let pending_usages = sqlx::query!(
        "SELECT spool_id, pending_weight, last_updated_at as 'last_updated_at: chrono::DateTime<Utc>' FROM pending_spool_usage WHERE pending_weight > 0"
    )
    .fetch_all(&mut conn as &mut sqlx::SqliteConnection)
    .await?;

    // 2. Iterate through the results and update Inventree.
    for usage in pending_usages {
        let spool_id = usage.spool_id;
        let pending_weight = usage.pending_weight;
        let last_updated_at: chrono::DateTime<Utc> = usage.last_updated_at;

        if (Utc::now() - last_updated_at).num_seconds() < 120 {
            debug!(
                "Skipping spool_id {} with pending_weight {} as it was updated less than 2 minutes ago",
                spool_id, pending_weight
            );
            continue;
        }

        // Construct the payload for the `remove_create` API call.
        let body = inventree::stock::RemoveCreateBody {
            notes: "Used via Spoolman Proxy (Batch Update)".to_string(),
            items: vec![inventree::stock::RemoveCreateInner {
                pk: StockItemId(spool_id as u64),
                quantity: format!("{:.5}", pending_weight),
            }],
        };

        // Check if the stock item still exists in Inventree.
        // If it doesn't exist, we should remove the pending usage entry.
        if !ctx.inv.stock().exists(StockItemId(spool_id as u64)).await? {
            sqlx::query!("DELETE FROM pending_spool_usage WHERE spool_id=?", spool_id)
                .execute(&mut conn as &mut sqlx::SqliteConnection)
                .await?;
            continue;
        }

        // Make the API call to Inventree.
        ctx.inv.stock().remove_create(&body).await?;

        // 3. Reset the pending weight in the database to 0.
        // This marks the transaction as successfully processed.
        sqlx::query!(
            "UPDATE pending_spool_usage SET pending_weight = 0 WHERE spool_id = ?",
            spool_id
        )
        .execute(&mut conn as &mut sqlx::SqliteConnection)
        .await?;
    }

    Ok(())
}
