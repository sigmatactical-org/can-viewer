//! Shop-side anomaly detection over replayed sessions: the same fixtures the
//! telemetry crate validates, driven through TelemetryReplayer + AnomalyEngine
//! exactly as `VehicleSession::poll_replay` does.

use std::path::PathBuf;

use sigma_diagnostics::vehicle::AnomalyRow;
use sigma_diagnostics::TelemetryReplayer;
use sigma_racer_telemetry::anomaly::AnomalyEngine;
use sigma_racer_telemetry::parse_ts_millis;

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

/// Replay a session file the way the Mechanic does, collecting event rows.
fn replay_session(name: &str) -> (Vec<AnomalyRow>, AnomalyEngine) {
    let mut replayer = TelemetryReplayer::open(fixture(name)).expect("fixture opens");
    let mut engine = AnomalyEngine::sigma_defaults();
    let mut rows = Vec::new();
    while let Some(msg) = replayer.step_message() {
        if msg.msg == "Event" {
            if let Some(ev) = engine.ingest_event(&msg) {
                rows.push(AnomalyRow::from_event(&ev, "bike"));
            }
        } else if let Some(ts) = parse_ts_millis(&msg.ts) {
            for ev in engine.observe(ts, replayer.state()) {
                rows.push(AnomalyRow::from_event(ev, "shop"));
            }
        }
    }
    (rows, engine)
}

#[test]
fn nominal_fixture_stays_quiet() {
    let (rows, engine) = replay_session("nominal-ride.jsonl");
    assert!(rows.is_empty(), "false positives: {rows:?}");
    assert!(engine.worst_active().is_none());
}

#[test]
fn faulty_fixture_raises_expected_alerts_in_order() {
    let (rows, engine) = replay_session("faulty-ride.jsonl");
    let raised: Vec<&str> = rows
        .iter()
        .filter(|r| r.active)
        .map(|r| r.id.as_str())
        .collect();
    assert_eq!(
        raised,
        vec!["coolant_rising", "coolant_overheat", "not_charging"],
        "alert sequence changed: {rows:?}"
    );
    // Overheat is Critical and latches: still the worst at end of session.
    assert_eq!(
        engine.worst_active().map(|(id, _)| id),
        Some("coolant_overheat")
    );
    // Rows carry display metadata for the UI.
    assert!(rows.iter().all(|r| r.source == "shop"));
    assert!(rows.iter().all(|r| !r.ts.is_empty()));
}

#[test]
fn replaying_twice_is_deterministic() {
    let (first, _) = replay_session("faulty-ride.jsonl");
    let (second, _) = replay_session("faulty-ride.jsonl");
    let key = |rows: &[AnomalyRow]| -> Vec<(String, String, String)> {
        rows.iter()
            .map(|r| (r.ts.clone(), r.id.clone(), r.severity_label.clone()))
            .collect()
    };
    assert_eq!(key(&first), key(&second));
}

#[test]
fn bike_event_for_already_raised_alert_is_deduplicated() {
    let (_, mut engine) = replay_session("faulty-ride.jsonl");
    // The bike reports the overheat the shop engine already holds active.
    let dup = sigma_racer_telemetry::protocol::Message::event(
        999,
        "2026-07-18T10:05:00.000Z".into(),
        "coolant_overheat",
        "Vehicle.OBD.CoolantTemperature",
        serde_json::Value::from(120),
        std::collections::HashMap::from([
            ("state".to_string(), serde_json::Value::from("raised")),
            ("severity".to_string(), serde_json::Value::from("CRITICAL")),
        ]),
    );
    assert!(engine.ingest_event(&dup).is_none(), "must deduplicate");
}
