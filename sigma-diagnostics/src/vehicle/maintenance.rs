//! Maintenance reset actions (protocol pending with Wingman).

/// Destructive / service actions exposed to Mechanic.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaintenanceAction {
    ResetServiceInterval,
    ResetOilLife,
    ClearMaintenanceWarning,
}

impl MaintenanceAction {
    pub fn label(self) -> &'static str {
        match self {
            Self::ResetServiceInterval => "Reset service interval",
            Self::ResetOilLife => "Reset oil life",
            Self::ClearMaintenanceWarning => "Clear maintenance warning",
        }
    }
}

pub trait MaintenanceService {
    fn perform(&self, action: MaintenanceAction) -> Result<String, String>;
}

/// Placeholder until Wingman defines write PDUs / UDS routines.
#[derive(Debug, Default)]
pub struct StubMaintenanceService;

impl MaintenanceService for StubMaintenanceService {
    fn perform(&self, action: MaintenanceAction) -> Result<String, String> {
        Err(format!(
            "{}: protocol pending (Wingman CAN/UDS not defined yet)",
            action.label()
        ))
    }
}
