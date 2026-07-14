use serde::Deserialize;

#[derive(Deserialize)]
pub struct Pagination {
    #[serde(default)]
    pub page: Option<i16>,
    #[serde(default)]
    pub expand: Option<bool>,
}
