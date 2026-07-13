use sigma_racer_telemetry::DEFAULT_TCP_PORT;

use crate::vehicle::transport::VehicleTransport;

/// Default Wingman telemetry relay port.
pub const DEFAULT_WIFI_PORT: u16 = DEFAULT_TCP_PORT;

/// Link parameters for SocketCAN or WiFi telemetry.
#[derive(Debug, Clone)]
pub struct VehicleLinkConfig {
    pub transport: VehicleTransport,
    pub interface: String,
    pub bitrate: u32,
    pub wifi_host: String,
    pub wifi_port: u16,
    pub use_m7_draft_dbc: bool,
    pub record_session: bool,
}

impl Default for VehicleLinkConfig {
    fn default() -> Self {
        Self {
            transport: VehicleTransport::SocketCan,
            interface: "can0".into(),
            bitrate: 500_000,
            wifi_host: String::new(),
            wifi_port: DEFAULT_WIFI_PORT,
            use_m7_draft_dbc: true,
            record_session: true,
        }
    }
}
