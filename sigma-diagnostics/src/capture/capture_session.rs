use std::sync::mpsc;

use crate::dto::LiveCaptureDisplay;

/// Handle for polling live capture updates from the UI thread.
pub struct CaptureSession {
    pub(super) update_rx: mpsc::Receiver<LiveCaptureDisplay>,
}

impl CaptureSession {
    /// Drain pending updates and return the latest display state.
    pub fn poll_update(&self) -> Option<LiveCaptureDisplay> {
        let mut latest = None;
        while let Ok(update) = self.update_rx.try_recv() {
            latest = Some(update);
        }
        latest
    }
}
