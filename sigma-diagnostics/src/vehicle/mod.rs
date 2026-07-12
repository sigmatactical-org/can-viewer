//! Vehicle-facing diagnostics for shop tools (Mechanic) over SocketCAN.
//!
//! Protocol PDUs for maintenance/settings writes are defined with Wingman firmware;
//! this module ships stubs and M7 draft decode for live diagnosis.

mod diagnosis;
mod logs;
mod m7;
mod maintenance;
mod ota;
mod session;
mod settings;

pub use diagnosis::{DiagnosisSnapshot, VitalSignal};
pub use logs::{LogExportRequest, request_log_export};
pub use m7::{M7_DRAFT_DBC, M7_DRAFT_DBC_NAME, load_m7_draft_dbc};
pub use maintenance::{MaintenanceAction, MaintenanceService, StubMaintenanceService};
pub use ota::{ChannelRelease, OtaConfig, fetch_channel_latest};
pub use session::{VehicleLinkConfig, VehicleSession, VehicleSessionStatus};
pub use settings::{SettingsService, StubSettingsService, VehicleSetting};
