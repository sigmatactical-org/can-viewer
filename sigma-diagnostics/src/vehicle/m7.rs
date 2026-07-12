//! Embedded M7 draft DBC (synced from sigma-racer-wingman schemas).

use crate::state::DiagnosticsState;
use dbc_rs::Dbc;

/// Canonical filename for the embedded M7 draft schema.
pub const M7_DRAFT_DBC_NAME: &str = "m7-draft.dbc";

/// Bytes of `m7-draft.dbc` (keep in sync with wingman `schemas/can/m7-draft.dbc`).
pub const M7_DRAFT_DBC: &str = include_str!("../../data/m7-draft.dbc");

/// Parse and install the embedded M7 draft DBC into diagnostics state.
pub fn load_m7_draft_dbc(state: &DiagnosticsState) -> Result<(), String> {
    let dbc = Dbc::parse(M7_DRAFT_DBC).map_err(|e| format!("Invalid M7 draft DBC: {e:?}"))?;
    state.set_dbc(dbc);
    *state.dbc_path.lock() = Some(M7_DRAFT_DBC_NAME.to_string());
    Ok(())
}
