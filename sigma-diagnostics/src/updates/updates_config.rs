/// Connection settings for the sigma-updates DBC catalog.
#[derive(Debug, Clone)]
pub struct UpdatesConfig {
    pub base_url: String,
}

impl UpdatesConfig {
    pub fn from_env() -> Self {
        Self {
            base_url: std::env::var("SIGMA_UPDATES_URL")
                .unwrap_or_else(|_| "http://updates.sigma.localtest.me:30080".into())
                .trim_end_matches('/')
                .to_owned(),
        }
    }

    pub fn list_dbc_url(&self) -> String {
        format!("{}/v1/dbc?page=1&per_page=500", self.base_url)
    }

    pub fn latest_dbc_url(&self) -> String {
        format!("{}/v1/dbc/latest", self.base_url)
    }

    pub fn download_url(&self, download_path: &str) -> String {
        if download_path.starts_with("http://") || download_path.starts_with("https://") {
            download_path.to_owned()
        } else {
            format!("{}{}", self.base_url, download_path)
        }
    }
}
