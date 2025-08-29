mod stock_list;
mod models;
mod stock_retrieve;
mod stock_remove_create;

pub use models::*;
pub use stock_list::*;
pub use stock_retrieve::*;
pub use stock_remove_create::*;

use crate::InventreeApiClient;

pub struct StockRepository(pub(crate) InventreeApiClient);

