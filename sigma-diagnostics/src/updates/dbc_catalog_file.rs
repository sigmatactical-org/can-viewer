use serde::Deserialize;

/// A single DBC file entry in the sigma-updates catalog.
#[derive(Debug, Clone, Deserialize)]
pub struct DbcCatalogFile {
    pub filename: String,
    pub name: String,
    pub size_bytes: u64,
    pub download_path: String,
}
