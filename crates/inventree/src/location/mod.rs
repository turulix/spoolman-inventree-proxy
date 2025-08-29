use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct LocationId(pub u64);

#[derive(Deserialize, Debug, Clone)]
pub struct LocationDetails {
    pub pk: LocationId,
    pub name: String,
    pub pathstring: String,
}
