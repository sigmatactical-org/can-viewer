use std::time::Duration;

use super::{ChannelRelease, OtaConfig};

/// Fetch the latest channel release metadata from sigma-updates.
pub fn fetch_channel_latest(cfg: &OtaConfig) -> Result<ChannelRelease, String> {
    let body = ureq::get(&cfg.latest_url())
        .timeout(Duration::from_secs(10))
        .call()
        .map_err(|e| format!("OTA catalog fetch failed: {e}"))?
        .into_string()
        .map_err(|e| format!("OTA catalog response: {e}"))?;
    serde_json::from_str(&body).map_err(|e| format!("OTA catalog JSON: {e}"))
}
