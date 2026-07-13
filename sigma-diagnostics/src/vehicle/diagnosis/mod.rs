//! Live diagnosis snapshot from decoded vehicle CAN frames or VSS telemetry.

mod diagnosis_snapshot;
mod vital_signal;

pub use diagnosis_snapshot::DiagnosisSnapshot;
pub use vital_signal::VitalSignal;
