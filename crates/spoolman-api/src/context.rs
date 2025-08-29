use inventree::InventreeApiClient;

#[derive(Clone)]
pub struct Context {
    pub inv: InventreeApiClient,
}
