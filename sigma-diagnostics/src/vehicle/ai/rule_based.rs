use super::advisor::DiagnosisAdvisor;
use super::reading::{DiagnosisReading, ReadingSeverity, ReadingSource};
use crate::vehicle::DiagnosisSnapshot;

/// Deterministic, offline advisor: threshold and presence checks over the
/// decoded signals. Always available, and the fallback when a model call fails.
#[derive(Debug, Default, Clone)]
pub struct RuleBasedAdvisor;

impl RuleBasedAdvisor {
    pub fn new() -> Self {
        Self
    }

    /// Findings + overall severity for a snapshot. Shared with the model
    /// advisor so the displayed severity is identical regardless of backend.
    pub(crate) fn assess(&self, snap: &DiagnosisSnapshot) -> Assessment {
        let mut findings = Vec::new();

        if let Some(c) = parse_leading_f64(&snap.coolant_c) {
            if c >= 115.0 {
                findings.push(Finding::new(
                    ReadingSeverity::Critical,
                    "Coolant temperature",
                    format!("{c:.0} °C — past the ~115 °C limit; the engine is overheating."),
                ));
            } else if c >= 105.0 {
                findings.push(Finding::new(
                    ReadingSeverity::Warning,
                    "Coolant temperature",
                    format!("{c:.0} °C — running hot (normal is under ~105 °C)."),
                ));
            }
        }

        if let Some(o) = parse_leading_f64(&snap.oil_c) {
            if o >= 135.0 {
                findings.push(Finding::new(
                    ReadingSeverity::Critical,
                    "Oil temperature",
                    format!("{o:.0} °C — past the ~135 °C limit."),
                ));
            } else if o >= 120.0 {
                findings.push(Finding::new(
                    ReadingSeverity::Warning,
                    "Oil temperature",
                    format!("{o:.0} °C — elevated (normal is under ~120 °C)."),
                ));
            }
        }

        if let Some(v) = battery_voltage(snap) {
            if v < 11.8 {
                findings.push(Finding::new(
                    ReadingSeverity::Critical,
                    "Battery voltage",
                    format!("{v:.1} V — very low; the bike is not charging."),
                ));
            } else if v < 12.4 {
                findings.push(Finding::new(
                    ReadingSeverity::Warning,
                    "Battery voltage",
                    format!(
                        "{v:.1} V — low; on a running engine expect ~14 V, so check the charging system."
                    ),
                ));
            }
        }

        if let Some(d) = parse_leading_f64(&snap.dtc_count) {
            let d = d as i64;
            if d > 0 {
                let sev = if d >= 3 {
                    ReadingSeverity::Critical
                } else {
                    ReadingSeverity::Warning
                };
                findings.push(Finding::new(
                    sev,
                    "Diagnostic trouble codes",
                    format!("{d} stored — read the codes on the ECU to confirm the fault(s)."),
                ));
            }
        }

        if let Some(ss) = normalize(&snap.side_stand) {
            if ss.eq_ignore_ascii_case("down") {
                findings.push(Finding::new(
                    ReadingSeverity::Info,
                    "Side stand",
                    "down — the engine will cut if the bike is put in gear.".into(),
                ));
            }
        }

        // Fold in the streaming anomaly detections (active alerts only, one
        // finding per alert id).
        let mut seen: Vec<&str> = Vec::new();
        for row in snap.anomalies.iter().filter(|r| r.active) {
            if seen.contains(&row.id.as_str()) {
                continue;
            }
            seen.push(&row.id);
            let severity = match row.severity_label.as_str() {
                "CRITICAL" => ReadingSeverity::Critical,
                "WARNING" => ReadingSeverity::Warning,
                _ => ReadingSeverity::Info,
            };
            findings.push(Finding::new(
                severity,
                &format!("Streaming alert {}", row.id),
                format!("{} (detected at {})", row.message, row.ts),
            ));
        }

        let severity = findings
            .iter()
            .fold(ReadingSeverity::Ok, |acc, f| acc.escalate(f.severity));
        Assessment { severity, findings }
    }
}

impl DiagnosisAdvisor for RuleBasedAdvisor {
    fn explain(&self, snap: &DiagnosisSnapshot, question: Option<&str>) -> DiagnosisReading {
        if !snap.connected {
            return DiagnosisReading {
                severity: ReadingSeverity::Info,
                headline: "Not connected.".into(),
                body: "Connect to the vehicle or replay a saved session to read live signals."
                    .into(),
                source: ReadingSource::RuleBased,
            };
        }

        let assessment = self.assess(snap);
        let (headline, mut body) = render_assessment(&assessment);
        if let Some(q) = question {
            body.push_str(&format!(
                "\n\n(Asked: \"{}\". Offline rule-based mode can't answer free-form questions — \
                 set SIGMA_AI_BASE_URL to an on-prem model to ask about specific signals or codes.)",
                q.trim()
            ));
        }
        DiagnosisReading {
            severity: assessment.severity,
            headline,
            body,
            source: ReadingSource::RuleBased,
        }
    }
}

/// A single flagged reading.
pub(crate) struct Finding {
    pub severity: ReadingSeverity,
    pub signal: String,
    pub detail: String,
}

impl Finding {
    fn new(severity: ReadingSeverity, signal: &str, detail: String) -> Self {
        Self {
            severity,
            signal: signal.to_string(),
            detail,
        }
    }
}

/// Overall severity plus the individual findings behind it.
pub(crate) struct Assessment {
    pub severity: ReadingSeverity,
    pub findings: Vec<Finding>,
}

/// Render a headline + plain-language body from an assessment. Shared by both
/// advisors so the offline and model paths read consistently.
pub(crate) fn render_assessment(a: &Assessment) -> (String, String) {
    if a.findings.is_empty() {
        return (
            "All monitored signals are within range.".into(),
            "RPM, coolant, oil, battery, and DTC count all read normal for the current state."
                .into(),
        );
    }

    let crit = a
        .findings
        .iter()
        .filter(|f| f.severity == ReadingSeverity::Critical)
        .count();
    let total = a.findings.len();
    let headline = match a.severity {
        ReadingSeverity::Critical => format!(
            "Critical — {crit} issue{} need attention before riding.",
            plural(crit)
        ),
        ReadingSeverity::Warning => {
            format!("Caution — {total} reading{} need a look.", plural(total))
        }
        _ => "Advisory — nothing critical, but note the following.".into(),
    };

    let mut body = String::new();
    for f in &a.findings {
        body.push_str(&format!(
            "• {} ({}): {}\n",
            f.signal,
            f.severity.label().to_ascii_lowercase(),
            f.detail
        ));
    }
    (headline, body.trim_end().to_string())
}

fn plural(n: usize) -> &'static str {
    if n == 1 {
        ""
    } else {
        "s"
    }
}

/// Battery voltage from the vitals list (name mentions "battery" or "voltage").
fn battery_voltage(snap: &DiagnosisSnapshot) -> Option<f64> {
    snap.vitals
        .iter()
        .find(|v| {
            let n = v.name.to_ascii_lowercase();
            n.contains("battery") || n.contains("voltage")
        })
        .and_then(|v| parse_leading_f64(&v.value))
}

/// Parse the leading numeric portion of a formatted value (e.g. "116 °C" → 116).
fn parse_leading_f64(s: &str) -> Option<f64> {
    let t = s.trim();
    let cut = t
        .find(|c: char| !(c.is_ascii_digit() || c == '.' || c == '-' || c == '+'))
        .unwrap_or(t.len());
    t[..cut].parse().ok()
}

fn normalize(s: &str) -> Option<String> {
    let t = s.trim();
    if t.is_empty() || t == "—" {
        None
    } else {
        Some(t.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vehicle::VitalSignal;

    fn snap() -> DiagnosisSnapshot {
        DiagnosisSnapshot {
            connected: true,
            ..DiagnosisSnapshot::default()
        }
    }

    #[test]
    fn parses_values_with_units() {
        assert_eq!(parse_leading_f64("116 °C"), Some(116.0));
        assert_eq!(parse_leading_f64("12.1"), Some(12.1));
        assert_eq!(parse_leading_f64("-90 deg"), Some(-90.0));
        assert_eq!(parse_leading_f64("N"), None);
    }

    #[test]
    fn flags_overheat_and_low_battery_as_critical() {
        let mut s = snap();
        s.coolant_c = "116 °C".into();
        s.dtc_count = "3".into();
        s.vitals = vec![VitalSignal {
            name: "Vehicle.ElectricalSystem.Battery.Voltage".into(),
            value: "11.4".into(),
            unit: "V".into(),
        }];
        let a = RuleBasedAdvisor::new().assess(&s);
        assert_eq!(a.severity, ReadingSeverity::Critical);
        assert!(a.findings.len() >= 3);
    }

    #[test]
    fn all_clear_when_nominal() {
        let mut s = snap();
        s.coolant_c = "88 °C".into();
        s.oil_c = "95 °C".into();
        s.dtc_count = "0".into();
        let a = RuleBasedAdvisor::new().assess(&s);
        assert_eq!(a.severity, ReadingSeverity::Ok);
        assert!(a.findings.is_empty());
    }

    #[test]
    fn disconnected_snapshot_is_info() {
        let r = RuleBasedAdvisor::new().explain(&DiagnosisSnapshot::default(), None);
        assert_eq!(r.severity, ReadingSeverity::Info);
        assert_eq!(r.source, ReadingSource::RuleBased);
    }

    #[test]
    fn active_streaming_anomaly_escalates_assessment() {
        use crate::vehicle::AnomalyRow;
        let mut s = snap();
        s.coolant_c = "88 °C".into();
        s.anomalies = vec![
            AnomalyRow {
                ts: "10:00:31.000".into(),
                id: "side_stand_interlock".into(),
                severity_label: "CRITICAL".into(),
                message: "Side stand down at 80 km/h in gear 3".into(),
                active: true,
                source: "shop".into(),
            },
            // Cleared alerts must not contribute.
            AnomalyRow {
                ts: "10:00:05.000".into(),
                id: "coolant_rising".into(),
                severity_label: "WARNING".into(),
                message: "cleared".into(),
                active: false,
                source: "shop".into(),
            },
        ];
        let a = RuleBasedAdvisor::new().assess(&s);
        assert_eq!(a.severity, ReadingSeverity::Critical);
        assert_eq!(a.findings.len(), 1, "only the active alert counts");
    }

    #[test]
    fn anomaly_free_snapshot_behavior_unchanged() {
        let mut s = snap();
        s.coolant_c = "88 °C".into();
        s.oil_c = "95 °C".into();
        s.dtc_count = "0".into();
        let a = RuleBasedAdvisor::new().assess(&s);
        assert_eq!(a.severity, ReadingSeverity::Ok);
    }
}
