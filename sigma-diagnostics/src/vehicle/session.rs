//! Active SocketCAN link to the vehicle for Mechanic.

use crate::capture::{self, CaptureSession};
use crate::state::DiagnosticsState;
use crate::vehicle::diagnosis::DiagnosisSnapshot;
use crate::vehicle::m7;
use parking_lot::Mutex;
use std::path::PathBuf;

/// SocketCAN link parameters (shop PC → bike diagnostic port).
#[derive(Debug, Clone)]
pub struct VehicleLinkConfig {
    pub interface: String,
    /// Bitrate hint for UI / docs (SocketCAN iface is configured externally).
    pub bitrate: u32,
    /// When true, prefer embedded M7 draft DBC on connect.
    pub use_m7_draft_dbc: bool,
}

impl Default for VehicleLinkConfig {
    fn default() -> Self {
        Self {
            interface: "can0".into(),
            bitrate: 500_000,
            use_m7_draft_dbc: true,
        }
    }
}

/// High-level connection status for the UI.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VehicleSessionStatus {
    Disconnected,
    Connecting,
    Connected,
    Error,
}

impl VehicleSessionStatus {
    pub fn label(self) -> &'static str {
        match self {
            Self::Disconnected => "Disconnected",
            Self::Connecting => "Connecting",
            Self::Connected => "Connected",
            Self::Error => "Error",
        }
    }
}

fn capture_log_path() -> PathBuf {
    std::env::temp_dir().join("sigma-racer-mechanic-capture.mf4")
}

/// Owns the live SocketCAN capture session used for diagnosis.
pub struct VehicleSession {
    config: Mutex<VehicleLinkConfig>,
    status: Mutex<VehicleSessionStatus>,
    last_error: Mutex<Option<String>>,
    capture: Mutex<Option<CaptureSession>>,
}

impl Default for VehicleSession {
    fn default() -> Self {
        Self::new()
    }
}

impl VehicleSession {
    pub fn new() -> Self {
        Self {
            config: Mutex::new(VehicleLinkConfig::default()),
            status: Mutex::new(VehicleSessionStatus::Disconnected),
            last_error: Mutex::new(None),
            capture: Mutex::new(None),
        }
    }

    pub fn status(&self) -> VehicleSessionStatus {
        *self.status.lock()
    }

    pub fn last_error(&self) -> Option<String> {
        self.last_error.lock().clone()
    }

    pub fn config(&self) -> VehicleLinkConfig {
        self.config.lock().clone()
    }

    pub fn set_config(&self, cfg: VehicleLinkConfig) {
        *self.config.lock() = cfg;
    }

    /// Connect: optionally load M7 DBC, start SocketCAN capture.
    pub fn connect(&self, state: &DiagnosticsState) -> Result<(), String> {
        let cfg = self.config.lock().clone();
        *self.status.lock() = VehicleSessionStatus::Connecting;
        *self.last_error.lock() = None;

        if let Err(e) = capture::stop_capture(state) {
            log::debug!("stop before connect: {e}");
        }
        *self.capture.lock() = None;

        if cfg.use_m7_draft_dbc {
            m7::load_m7_draft_dbc(state)?;
        }

        let log_path = capture_log_path();
        let log_str = log_path.to_string_lossy().to_string();

        match capture::start_capture(&cfg.interface, &log_str, false, None, state) {
            Ok(session) => {
                *self.capture.lock() = Some(session);
                *self.status.lock() = VehicleSessionStatus::Connected;
                Ok(())
            }
            Err(e) => {
                *self.status.lock() = VehicleSessionStatus::Error;
                *self.last_error.lock() = Some(e.clone());
                Err(e)
            }
        }
    }

    pub fn disconnect(&self, state: &DiagnosticsState) {
        let _ = capture::stop_capture(state);
        *self.capture.lock() = None;
        *self.status.lock() = VehicleSessionStatus::Disconnected;
    }

    pub fn is_connected(&self, state: &DiagnosticsState) -> bool {
        matches!(*self.status.lock(), VehicleSessionStatus::Connected)
            && capture::is_capture_running(state)
    }

    /// Poll capture and build a diagnosis snapshot.
    pub fn poll_diagnosis(&self, state: &DiagnosticsState) -> DiagnosisSnapshot {
        let connected = self.is_connected(state);
        if !connected {
            let reason = self
                .last_error
                .lock()
                .clone()
                .unwrap_or_else(|| "Not connected".into());
            return DiagnosisSnapshot::disconnected(&reason);
        }

        let capture = self.capture.lock();
        let Some(session) = capture.as_ref() else {
            return DiagnosisSnapshot::disconnected("No capture session");
        };

        match session.poll_update() {
            Some(display) => DiagnosisSnapshot::from_live_display(&display, true),
            None => DiagnosisSnapshot {
                connected: true,
                status: "Waiting for frames".into(),
                ..DiagnosisSnapshot::default()
            },
        }
    }
}
