//! Shop-side OTA channel catalog (download only; RAUC apply stays on-device).

mod channel_release;
mod fetch;
mod ota_config;

pub use channel_release::ChannelRelease;
pub use fetch::fetch_channel_latest;
pub use ota_config::OtaConfig;
