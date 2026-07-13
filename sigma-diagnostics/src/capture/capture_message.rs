use crate::dto::{CanErrorDto, CanFrameDto};

/// A frame or error handed from the socket-reader thread to the display thread.
pub(super) enum CaptureMessage {
    Frame(CanFrameDto),
    Error(CanErrorDto),
}
