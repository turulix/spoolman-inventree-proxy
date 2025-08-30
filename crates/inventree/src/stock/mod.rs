mod models;
mod stock_list;
mod stock_remove_create;
mod stock_retrieve;
mod stock_exists;

pub use models::*;
pub use stock_list::*;
pub use stock_remove_create::*;
pub use stock_retrieve::*;

use crate::InventreeApiClient;

pub struct StockRepository(pub(crate) InventreeApiClient);
