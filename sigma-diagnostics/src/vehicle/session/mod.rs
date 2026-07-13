//! Active vehicle link for Mechanic (SocketCAN or Wingman WiFi telemetry).

mod vehicle_link_config;
mod vehicle_session;
mod vehicle_session_status;

pub use vehicle_link_config::{VehicleLinkConfig, DEFAULT_WIFI_PORT};
pub use vehicle_session::VehicleSession;
pub use vehicle_session_status::VehicleSessionStatus;
