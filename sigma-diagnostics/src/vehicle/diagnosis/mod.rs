//! Live diagnosis snapshot from decoded vehicle CAN frames or VSS telemetry.

mod anomaly_row;
mod diagnosis_snapshot;
mod vital_signal;

pub use anomaly_row::AnomalyRow;
pub use diagnosis_snapshot::DiagnosisSnapshot;
pub use vital_signal::VitalSignal;
