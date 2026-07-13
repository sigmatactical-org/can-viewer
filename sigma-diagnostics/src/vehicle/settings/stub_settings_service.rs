use super::{SettingsService, VehicleSetting};

/// Placeholder until Wingman exposes a config service over CAN.
#[derive(Debug, Default)]
pub struct StubSettingsService;

impl SettingsService for StubSettingsService {
    fn list(&self) -> Result<Vec<VehicleSetting>, String> {
        Ok(vec![
            VehicleSetting {
                key: "PerformanceMode".into(),
                value: "(live — see Diagnosis)".into(),
                read_only: true,
            },
            VehicleSetting {
                key: "protocol".into(),
                value: "pending".into(),
                read_only: true,
            },
        ])
    }

    fn write(&self, key: &str, _value: &str) -> Result<(), String> {
        Err(format!(
            "Write {key}: protocol pending (Wingman settings service not defined yet)"
        ))
    }
}
