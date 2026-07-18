use sigma_racer_telemetry::anomaly::{AnomalyEvent, Edge};

/// One anomaly event for the Mechanic diagnosis panel.
#[derive(Debug, Clone, Default)]
pub struct AnomalyRow {
    /// Wall-clock time of day the edge fired (HH:MM:SS.mmm, UTC).
    pub ts: String,
    /// Stable alert id, e.g. `coolant_overheat`.
    pub id: String,
    /// Uppercase severity label (ADVISORY / WARNING / CRITICAL).
    pub severity_label: String,
    pub message: String,
    /// Whether the alert is still active at the latest snapshot.
    pub active: bool,
    /// Where the detection ran: `shop` (local engine) or `bike` (Event msg).
    pub source: String,
}

impl AnomalyRow {
    /// Build a display row from an engine event.
    pub fn from_event(ev: &AnomalyEvent, source: &str) -> Self {
        let message = match ev.edge {
            Edge::Raised => ev.message.clone(),
            Edge::Cleared => format!("cleared — {}", ev.message),
        };
        Self {
            ts: format_time_of_day(ev.ts_ms),
            id: ev.id.clone(),
            severity_label: ev.severity.label().into(),
            message,
            active: ev.edge == Edge::Raised,
            source: source.into(),
        }
    }
}

/// HH:MM:SS.mmm (UTC) from epoch milliseconds, without a date dependency.
fn format_time_of_day(ts_ms: i64) -> String {
    let ms_of_day = ts_ms.rem_euclid(86_400_000);
    let (h, rem) = (ms_of_day / 3_600_000, ms_of_day % 3_600_000);
    let (m, rem) = (rem / 60_000, rem % 60_000);
    let (s, ms) = (rem / 1_000, rem % 1_000);
    format!("{h:02}:{m:02}:{s:02}.{ms:03}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_time_of_day() {
        // 2026-07-18T10:00:31.250Z
        assert_eq!(format_time_of_day(1_784_455_231_250), "10:00:31.250");
        assert_eq!(format_time_of_day(0), "00:00:00.000");
    }
}
