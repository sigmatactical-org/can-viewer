use std::time::Duration;

use super::{ChannelRelease, OtaConfig};

/// Fetch the latest channel release metadata from sigma-updates.
pub fn fetch_channel_latest(cfg: &OtaConfig) -> Result<ChannelRelease, String> {
    let body = ureq::get(&cfg.latest_url())
        .config()
        .timeout_global(Some(Duration::from_secs(10)))
        .build()
        .call()
        .map_err(|e| format!("OTA catalog fetch failed: {e}"))?
        .body_mut()
        .read_to_string()
        .map_err(|e| format!("OTA catalog response: {e}"))?;
    serde_json::from_str(&body).map_err(|e| format!("OTA catalog JSON: {e}"))
}
