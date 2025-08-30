use inventree::InventreeApiClient;
use crate::db::DbClient;

#[derive(Clone)]
pub struct Context {
    pub inv: InventreeApiClient,
    pub db: DbClient,
}
