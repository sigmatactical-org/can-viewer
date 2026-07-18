//! On-prem natural-language diagnosis advisor.
//!
//! Turns a [`DiagnosisSnapshot`](crate::vehicle::DiagnosisSnapshot) of decoded
//! live signals into a plain-language read for a shop technician. Severity and
//! which signals are out of range are always computed deterministically
//! (rule-based, offline); an optional on-prem language model only phrases the
//! explanation. When no model is configured — or a call fails — the rule-based
//! prose is used verbatim, so the panel always works without network access.

mod advisor;
mod config;
mod model;
mod reading;
mod rule_based;

use std::sync::Arc;

pub use advisor::DiagnosisAdvisor;
pub use config::AiConfig;
pub use model::ModelAdvisor;
pub use reading::{DiagnosisReading, ReadingSeverity, ReadingSource};
pub use rule_based::RuleBasedAdvisor;

/// Build the advisor implied by `cfg`: the on-prem model when one is
/// configured, otherwise the offline rule-based advisor.
pub fn build_diagnosis_advisor(cfg: AiConfig) -> Arc<dyn DiagnosisAdvisor> {
    if cfg.enabled() {
        Arc::new(ModelAdvisor::new(cfg))
    } else {
        Arc::new(RuleBasedAdvisor::new())
    }
}
