//! On-bike log export (bulk MDF4 pull deferred; CAN trigger stub).

#[derive(Debug, Clone, Default)]
pub struct LogExportRequest {
    pub note: String,
}

/// Ask the ECU to prepare a log for export. Full file transfer is not over CAN.
pub fn request_log_export(_req: &LogExportRequest) -> Result<String, String> {
    Err(
        "Request log from ECU: protocol pending — import local MDF4 via Analysis → MDF4 for now"
            .into(),
    )
}
