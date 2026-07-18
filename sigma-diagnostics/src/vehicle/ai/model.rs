use std::time::Duration;

use super::advisor::DiagnosisAdvisor;
use super::config::AiConfig;
use super::reading::{DiagnosisReading, ReadingSource};
use super::rule_based::{render_assessment, Assessment, RuleBasedAdvisor};
use crate::vehicle::DiagnosisSnapshot;

const SYSTEM_PROMPT: &str =
    "You are a diagnostic assistant for a Sigma Racer M7 sport motorcycle, \
helping a shop technician. You are given live decoded CAN signals and the results of automated \
range checks. Explain in clear, plain language a technician would use. Be concise — a few short \
sentences or bullet points. Ground every statement in the provided signals; never invent signals, \
codes, or values that are not listed. Do not restate a severity verdict (the tool shows that \
separately). If nothing is wrong, say so briefly. Prefer actionable guidance: what to check first \
and why.";

/// On-prem model advisor.
///
/// Grounds a local language model on the decoded signals and the rule-based
/// findings, and uses the model only to phrase the explanation. The displayed
/// severity always comes from the deterministic assessment; any request error
/// falls back to the rule-based prose.
pub struct ModelAdvisor {
    cfg: AiConfig,
    rules: RuleBasedAdvisor,
}

impl ModelAdvisor {
    pub fn new(cfg: AiConfig) -> Self {
        Self {
            cfg,
            rules: RuleBasedAdvisor::new(),
        }
    }

    fn request_prose(&self, prompt: &str) -> Result<String, String> {
        let body = serde_json::json!({
            "model": self.cfg.model,
            "temperature": 0.2,
            "stream": false,
            "messages": [
                { "role": "system", "content": SYSTEM_PROMPT },
                { "role": "user", "content": prompt },
            ],
        });

        let payload =
            serde_json::to_string(&body).map_err(|e| format!("AI request encode: {e}"))?;
        let base = ureq::post(&self.cfg.chat_url()).header("Content-Type", "application/json");
        let base = match &self.cfg.api_key {
            Some(key) => base.header("Authorization", format!("Bearer {key}")),
            None => base,
        };
        let mut resp = base
            .config()
            .timeout_global(Some(Duration::from_secs(self.cfg.timeout_secs)))
            .build()
            .send(payload.as_bytes())
            .map_err(|e| format!("AI request failed: {e}"))?;

        let text = resp
            .body_mut()
            .read_to_string()
            .map_err(|e| format!("AI response read: {e}"))?;
        let parsed: serde_json::Value =
            serde_json::from_str(&text).map_err(|e| format!("AI response JSON: {e}"))?;
        let content = parsed["choices"][0]["message"]["content"]
            .as_str()
            .ok_or("AI response missing choices[0].message.content")?
            .trim()
            .to_string();
        if content.is_empty() {
            return Err("AI returned empty content".into());
        }
        Ok(content)
    }
}

impl DiagnosisAdvisor for ModelAdvisor {
    fn explain(&self, snap: &DiagnosisSnapshot, question: Option<&str>) -> DiagnosisReading {
        // Disconnected / no-signal cases are handled uniformly by the rules.
        if !snap.connected {
            return self.rules.explain(snap, question);
        }

        let assessment = self.rules.assess(snap);
        let (headline, rule_body) = render_assessment(&assessment);
        let prompt = build_prompt(snap, &assessment, question);

        match self.request_prose(&prompt) {
            Ok(prose) => DiagnosisReading {
                severity: assessment.severity,
                headline,
                body: prose,
                source: ReadingSource::Model,
            },
            Err(e) => {
                log::warn!("AI advisor fell back to rule-based: {e}");
                let body = format!(
                    "{rule_body}\n\n(On-prem model unavailable — showing the offline rule-based read.)"
                );
                DiagnosisReading {
                    severity: assessment.severity,
                    headline,
                    body,
                    source: ReadingSource::RuleBased,
                }
            }
        }
    }
}

/// Build the grounded user prompt: the decoded signals, the automated findings,
/// and either the technician's question or the default "explain this" ask.
fn build_prompt(snap: &DiagnosisSnapshot, a: &Assessment, question: Option<&str>) -> String {
    let mut s = String::new();
    s.push_str("Live decoded signals from the Sigma Racer M7 (via the m7-draft DBC):\n");
    push_kv(&mut s, "Engine RPM", &snap.rpm);
    push_kv(&mut s, "Coolant temp", &snap.coolant_c);
    push_kv(&mut s, "Oil temp", &snap.oil_c);
    push_kv(&mut s, "Gear", &snap.gear);
    push_kv(&mut s, "Performance mode", &snap.performance_mode);
    push_kv(&mut s, "Side stand", &snap.side_stand);
    push_kv(&mut s, "Stored DTC count", &snap.dtc_count);
    for v in &snap.vitals {
        if !v.value.trim().is_empty() {
            s.push_str(&format!("- {}: {} {}\n", v.name, v.value.trim(), v.unit));
        }
    }
    s.push('\n');

    if a.findings.is_empty() {
        s.push_str("Automated checks (range + deterministic streaming anomaly detectors) found nothing out of range.\n");
    } else {
        s.push_str(
            "Automated checks (range + deterministic streaming anomaly detectors) flagged:\n",
        );
        for f in &a.findings {
            s.push_str(&format!(
                "- [{}] {}: {}\n",
                f.severity.label(),
                f.signal,
                f.detail
            ));
        }
    }
    s.push('\n');

    match question {
        Some(q) => s.push_str(&format!(
            "The technician asks: \"{}\"\nAnswer their question using only the signals above.",
            q.trim()
        )),
        None => s.push_str(
            "Explain in plain language what these readings mean for the technician, focused on the \
             flagged items and what to check first.",
        ),
    }
    s
}

fn push_kv(s: &mut String, label: &str, value: &str) {
    let v = value.trim();
    if !v.is_empty() && v != "—" {
        s.push_str(&format!("- {label}: {v}\n"));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vehicle::ReadingSeverity;

    #[test]
    fn falls_back_to_rules_when_endpoint_unreachable() {
        // Discard port on loopback: the request fails fast without leaving the host.
        let advisor = ModelAdvisor::new(AiConfig {
            base_url: "http://127.0.0.1:9".into(),
            model: "test".into(),
            api_key: None,
            timeout_secs: 2,
        });
        let snap = DiagnosisSnapshot {
            connected: true,
            coolant_c: "118 °C".into(),
            ..DiagnosisSnapshot::default()
        };
        let reading = advisor.explain(&snap, None);
        // Severity is deterministic; prose falls back to the offline read.
        assert_eq!(reading.severity, ReadingSeverity::Critical);
        assert_eq!(reading.source, ReadingSource::RuleBased);
        assert!(reading.body.contains("rule-based read"));
    }
}
