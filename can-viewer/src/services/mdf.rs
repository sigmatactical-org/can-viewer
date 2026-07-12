//! MDF4 services with desktop session persistence.

use crate::state::AppState;
use sigma_diagnostics::dto::{CanFrameDto, DecodedSignalDto};

/// Load an MDF4 file and extract CAN frames; persist path in the session.
pub fn load_mdf4(
    path: &str,
    state: &AppState,
) -> Result<(Vec<CanFrameDto>, Vec<DecodedSignalDto>), String> {
    let result = sigma_diagnostics::load_mdf4(path, &state.diag)?;
    if let Err(e) = state.session.lock().set_mdf4_path(Some(path.to_string())) {
        log::warn!("Failed to save MDF4 path: {e}");
    }
    Ok(result)
}

pub use sigma_diagnostics::parse_can_dataframe;
