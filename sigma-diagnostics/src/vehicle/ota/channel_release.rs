use serde::Deserialize;

/// Latest release metadata for an OTA channel.
#[derive(Debug, Clone, Deserialize)]
pub struct ChannelRelease {
    pub channel: String,
    pub version: String,
    #[serde(default)]
    pub notes: String,
    #[serde(default)]
    pub install: String,
    #[serde(default)]
    pub bundle_url: String,
}
