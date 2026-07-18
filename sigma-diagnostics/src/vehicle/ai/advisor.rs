use super::reading::DiagnosisReading;
use crate::vehicle::DiagnosisSnapshot;

/// Produces a plain-language read from a decoded diagnosis snapshot.
///
/// Implementations must be `Send + Sync` so the shop UI can run `explain` on a
/// worker thread and post the result back to the event loop.
pub trait DiagnosisAdvisor: Send + Sync {
    /// Explain the current snapshot, optionally answering a technician question.
    fn explain(&self, snapshot: &DiagnosisSnapshot, question: Option<&str>) -> DiagnosisReading;
}
