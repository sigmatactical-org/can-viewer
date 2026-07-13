use serde::Deserialize;

use super::DbcCatalogFile;

/// Paged response body from the sigma-updates DBC catalog endpoint.
#[derive(Debug, Clone, Deserialize)]
pub struct DbcCatalogResponse {
    pub files: Vec<DbcCatalogFile>,
    pub total: usize,
    pub page: u32,
    pub per_page: u32,
    pub total_pages: u32,
    #[serde(default)]
    pub query: String,
}
