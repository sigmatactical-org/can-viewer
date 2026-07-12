//! Shared UI helpers.

use crate::config::SessionConfig;
use slint::{ModelRc, VecModel};
use std::path::{Path, PathBuf};

pub fn vec_model<T: Clone + 'static>(items: &[T]) -> ModelRc<T> {
    let model = VecModel::default();
    for item in items {
        model.push(item.clone());
    }
    ModelRc::new(model)
}

pub fn format_can_id(id: u32, extended: bool) -> String {
    if extended {
        format!("0x{id:08X}")
    } else {
        format!("0x{id:03X}")
    }
}

pub fn format_data_hex(data: &[u8]) -> String {
    data.iter()
        .map(|b| format!("{b:02X}"))
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn parse_hex_id(s: &str) -> Option<u32> {
    let s = s.trim().trim_start_matches("0x").trim_start_matches("0X");
    u32::from_str_radix(s, 16).ok()
}

pub fn parse_can_ids(text: &str) -> Vec<u32> {
    text.split([',', ' ', ';'])
        .filter_map(parse_hex_id)
        .collect()
}

pub fn set_status(ui: &crate::SigmaDiagnostics, message: &str) {
    ui.set_status_text(message.into());
}

/// Directory that is guaranteed to exist for file dialogs (avoids portal errors
/// when the last-used path was under the old `can-viewer` tree).
fn dialog_start_dir() -> PathBuf {
    let session = SessionConfig::load_raw();
    for candidate in [
        session
            .mdf4_path
            .as_ref()
            .and_then(|p| Path::new(p).parent().map(|d| d.to_path_buf())),
        session
            .dbc_path
            .as_ref()
            .and_then(|p| Path::new(p).parent().map(|d| d.to_path_buf())),
        SessionConfig::config_dir(),
        dirs::home_dir(),
    ]
    .into_iter()
    .flatten()
    {
        if candidate.is_dir() {
            return candidate;
        }
    }
    std::env::temp_dir()
}

pub fn pick_open_file(title: &str, extensions: &[(&str, &[&str])]) -> Option<String> {
    let mut dialog = rfd::FileDialog::new()
        .set_title(title)
        .set_directory(dialog_start_dir());
    for (name, exts) in extensions {
        dialog = dialog.add_filter(*name, exts);
    }
    dialog.pick_file().map(|p| p.display().to_string())
}

pub fn pick_save_file(title: &str, extensions: &[(&str, &[&str])]) -> Option<String> {
    let mut dialog = rfd::FileDialog::new()
        .set_title(title)
        .set_directory(dialog_start_dir());
    for (name, exts) in extensions {
        dialog = dialog.add_filter(*name, exts);
    }
    dialog.save_file().map(|p| p.display().to_string())
}
