use crate::db::DbClient;
use inventree::InventreeApiClient;

#[derive(Clone)]
pub struct Context {
    pub inv: InventreeApiClient,
    pub db: DbClient,
}
