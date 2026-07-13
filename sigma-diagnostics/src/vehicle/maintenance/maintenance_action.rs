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
