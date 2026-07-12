//! DBC services with desktop session persistence.

use crate::state::AppState;
use sigma_diagnostics::dto::DbcInfo;

/// Load and parse a DBC file from disk; persist path in the session.
pub fn load_dbc(path: &str, state: &AppState) -> Result<String, String> {
    let result = sigma_diagnostics::load_dbc(path, &state.diag)?;
    if let Err(e) = state.session.lock().set_dbc_path(Some(path.to_string())) {
        log::warn!("Failed to save session: {e}");
    }
    Ok(result)
}

/// Clear the loaded DBC and session path.
pub fn clear_dbc(state: &AppState) -> Result<(), String> {
    sigma_diagnostics::clear_dbc(&state.diag)?;
    if let Err(e) = state.session.lock().set_dbc_path(None) {
        log::warn!("Failed to save session: {e}");
    }
    Ok(())
}

/// Save DBC content to a file after validation; persist path in the session.
pub fn save_dbc_content(path: &str, content: &str, state: &AppState) -> Result<(), String> {
    sigma_diagnostics::save_dbc_content(path, content, &state.diag)?;
    if let Err(e) = state.session.lock().set_dbc_path(Some(path.to_string())) {
        log::warn!("Failed to save session: {e}");
    }
    Ok(())
}

/// Save a [`DbcInfo`] structure to disk.
pub fn save_dbc_info(path: &str, info: &DbcInfo, state: &AppState) -> Result<(), String> {
    let content = sigma_diagnostics::dbc_export::export_dbc_info_to_string(info);
    save_dbc_content(path, &content, state)
}
