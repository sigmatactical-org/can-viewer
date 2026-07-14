//! Sigma Racer instrument cluster — thin Slint UI shell.
//!
//! SPDX-License-Identifier: GPL-3.0-only
//!
//! Licensing boundary: this crate links Slint under the GPL-3.0-only
//! option and is itself GPL-3.0-only. All reusable logic (CAN decode,
//! protocols, vehicle state) belongs in the MIT OR Apache-2.0
//! `sigma-diagnostics` crate — this shell only maps domain types onto
//! Slint properties. See README.md before adding code here.

// deny (not forbid): the Slint-generated UI module carries its own
// scoped allow(unsafe_code) for vtable statics.
#![deny(unsafe_code)]

use sigma_diagnostics::DiagnosisSnapshot;

slint::include_modules!();

/// Map a domain snapshot onto the cluster window properties.
fn apply_snapshot(ui: &ClusterWindow, snap: &DiagnosisSnapshot) {
    ui.set_connected(snap.connected);
    ui.set_rpm(or_dash(&snap.rpm));
    ui.set_gear(if snap.gear.is_empty() {
        "N".into()
    } else {
        snap.gear.as_str().into()
    });
    ui.set_mode(or_dash(&snap.performance_mode));
    ui.set_coolant(or_dash(&snap.coolant_c));
    ui.set_oil(or_dash(&snap.oil_c));
    ui.set_dtc_count(or_dash(&snap.dtc_count));
    ui.set_status(snap.status.as_str().into());
}

fn or_dash(s: &str) -> slint::SharedString {
    if s.is_empty() {
        "—".into()
    } else {
        s.into()
    }
}

fn main() -> Result<(), slint::PlatformError> {
    env_logger::init();

    let ui = ClusterWindow::new()?;
    // Vehicle-bus wiring lands here (SocketCAN → DiagnosisSnapshot via
    // sigma-diagnostics); until then, render the disconnected state.
    apply_snapshot(
        &ui,
        &DiagnosisSnapshot {
            status: "Waiting for bus…".into(),
            ..DiagnosisSnapshot::default()
        },
    );
    ui.run()
}
