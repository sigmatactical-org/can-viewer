use super::VehicleSetting;

/// Reads and writes vehicle settings (protocol pending with Wingman).
pub trait SettingsService {
    fn list(&self) -> Result<Vec<VehicleSetting>, String>;
    fn write(&self, key: &str, value: &str) -> Result<(), String>;
}
