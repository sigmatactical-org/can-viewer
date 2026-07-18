//! AI Diagnosis panel: turns the live diagnosis snapshot into a plain-language
//! read via the on-prem advisor (or the offline rule-based fallback).

use crate::state::AppState;
use crate::SigmaRacerMechanic;
use sigma_diagnostics::{build_diagnosis_advisor, AiConfig, DiagnosisAdvisor};
use slint::Weak;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

/// Controller for the plain-language read panel in the Diagnosis tab.
pub struct AiController {
    state: Arc<AppState>,
    ui: Weak<SigmaRacerMechanic>,
    advisor: Arc<dyn DiagnosisAdvisor>,
    backend_label: String,
    busy: Arc<AtomicBool>,
}

impl AiController {
    /// Controller bound to the shared state and UI handle. Reads the advisor
    /// backend from the environment (`SIGMA_AI_BASE_URL`, etc.).
    pub fn new(state: Arc<AppState>, ui: Weak<SigmaRacerMechanic>) -> Self {
        let cfg = AiConfig::from_env();
        let backend_label = cfg.endpoint_label();
        let advisor = build_diagnosis_advisor(cfg);
        Self {
            state,
            ui,
            advisor,
            backend_label,
            busy: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Hook the AI panel callbacks.
    pub fn wire(self: Rc<Self>, ui: &SigmaRacerMechanic) {
        ui.on_ai_explain({
            let t = self.clone();
            move || t.explain()
        });
    }

    /// Seed the panel labels before any request.
    pub fn init(&self) {
        if let Some(ui) = self.ui.upgrade() {
            ui.set_ai_source(self.backend_label.clone().into());
            ui.set_ai_severity("".into());
            ui.set_ai_headline("Press Explain to read the current signals.".into());
            ui.set_ai_body("".into());
            ui.set_ai_status("".into());
            ui.set_ai_busy(false);
        }
    }

    fn explain(&self) {
        // Guard against overlapping requests.
        if self.busy.swap(true, Ordering::SeqCst) {
            return;
        }
        let Some(ui) = self.ui.upgrade() else {
            self.busy.store(false, Ordering::SeqCst);
            return;
        };

        let snapshot = self.state.vehicle.poll_diagnosis(&self.state.analysis.diag);
        let question = {
            let q = ui.get_ai_question().to_string();
            let q = q.trim().to_string();
            if q.is_empty() {
                None
            } else {
                Some(q)
            }
        };

        ui.set_ai_busy(true);
        ui.set_ai_status("Reading signals…".into());

        let advisor = self.advisor.clone();
        let weak = self.ui.clone();
        let busy = self.busy.clone();
        // The advisor may block on a network call; run it off the UI thread and
        // post the result back to the event loop.
        std::thread::spawn(move || {
            let reading = advisor.explain(&snapshot, question.as_deref());
            let severity = reading.severity.label().to_string();
            let headline = reading.headline;
            let body = reading.body;
            let source = reading.source.label().to_string();
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(ui) = weak.upgrade() {
                    ui.set_ai_severity(severity.into());
                    ui.set_ai_headline(headline.into());
                    ui.set_ai_body(body.into());
                    ui.set_ai_source(source.into());
                    ui.set_ai_status("".into());
                    ui.set_ai_busy(false);
                }
                busy.store(false, Ordering::SeqCst);
            });
        });
    }
}
