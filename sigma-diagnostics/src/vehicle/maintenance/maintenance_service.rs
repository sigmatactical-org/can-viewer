use super::MaintenanceAction;

/// Performs maintenance reset actions (protocol pending with Wingman).
pub trait MaintenanceService {
    fn perform(&self, action: MaintenanceAction) -> Result<String, String>;
}
