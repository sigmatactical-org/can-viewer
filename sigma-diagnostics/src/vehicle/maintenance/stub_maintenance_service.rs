use super::{MaintenanceAction, MaintenanceService};

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
