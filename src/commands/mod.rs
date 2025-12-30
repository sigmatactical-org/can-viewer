//! Tauri command handlers.
//!
//! Each submodule contains related commands grouped by functionality.
//!
//! # Extension Point
//!
//! Pro versions can add commands by:
//! 1. Creating a `pro/` module with additional commands
//! 2. Using the `all_commands!` macro with additional handlers

mod capture;
mod dbc;
pub mod filter;
mod init;
pub mod mdf;

pub use capture::*;
pub use dbc::*;
pub use filter::*;
pub use init::*;
pub use mdf::*;

/// Generate the base command handler.
///
/// This macro makes it easy for pro versions to extend with additional commands:
///
/// ```ignore
/// // In pro/mod.rs:
/// pub fn invoke_handler() -> impl Fn(tauri::ipc::Invoke) -> bool {
///     tauri::generate_handler![
///         // Base commands
///         commands::load_dbc,
///         commands::clear_dbc,
///         // ... all base commands ...
///         // Pro commands
///         pro::commands::sync_to_cloud,
///         pro::commands::run_script,
///     ]
/// }
/// ```
#[macro_export]
macro_rules! base_commands {
    () => {
        tauri::generate_handler![
            can_viewer::commands::load_dbc,
            can_viewer::commands::clear_dbc,
            can_viewer::commands::get_dbc_path,
            can_viewer::commands::save_dbc_content,
            can_viewer::commands::update_dbc_content,
            can_viewer::commands::decode_single_frame,
            can_viewer::commands::decode_frames,
            can_viewer::commands::get_dbc_info,
            can_viewer::commands::get_dbc_specification,
            can_viewer::commands::load_mdf4,
            can_viewer::commands::export_logs,
            can_viewer::commands::list_can_interfaces,
            can_viewer::commands::start_capture,
            can_viewer::commands::stop_capture,
            can_viewer::commands::is_capture_running,
            can_viewer::commands::get_initial_files,
            // Frame filter commands (all computation in Rust)
            can_viewer::commands::filter_frames,
            can_viewer::commands::calculate_frame_stats,
            can_viewer::commands::get_message_counts,
            can_viewer::commands::detect_dlc,
        ]
    };
    // Extended version: base + additional commands
    ($($extra:path),+ $(,)?) => {
        tauri::generate_handler![
            can_viewer::commands::load_dbc,
            can_viewer::commands::clear_dbc,
            can_viewer::commands::get_dbc_path,
            can_viewer::commands::save_dbc_content,
            can_viewer::commands::update_dbc_content,
            can_viewer::commands::decode_single_frame,
            can_viewer::commands::decode_frames,
            can_viewer::commands::get_dbc_info,
            can_viewer::commands::get_dbc_specification,
            can_viewer::commands::load_mdf4,
            can_viewer::commands::export_logs,
            can_viewer::commands::list_can_interfaces,
            can_viewer::commands::start_capture,
            can_viewer::commands::stop_capture,
            can_viewer::commands::is_capture_running,
            can_viewer::commands::get_initial_files,
            // Frame filter commands (all computation in Rust)
            can_viewer::commands::filter_frames,
            can_viewer::commands::calculate_frame_stats,
            can_viewer::commands::get_message_counts,
            can_viewer::commands::detect_dlc,
            $($extra),+
        ]
    };
}
