//! SocketCAN capture services (Linux only).
//!
//! [`CaptureSession`] is the UI-side handle; the capture service functions run
//! the socket-reader and display threads that feed it.

#[cfg(target_os = "linux")]
mod capture_message;
mod capture_session;
mod service;

pub use capture_session::CaptureSession;
pub use service::{is_capture_running, list_can_interfaces, start_capture, stop_capture};
