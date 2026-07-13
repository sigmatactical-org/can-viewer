//! Vehicle settings read/write (protocol pending with Wingman).

mod settings_service;
mod stub_settings_service;
mod vehicle_setting;

pub use settings_service::SettingsService;
pub use stub_settings_service::StubSettingsService;
pub use vehicle_setting::VehicleSetting;
