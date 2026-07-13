use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

use sigma_racer_telemetry::protocol::Message;
use sigma_racer_telemetry::VehicleState;

/// Offline replay of a saved NDJSON session.
pub struct TelemetryReplayer {
    lines: Vec<String>,
    index: usize,
    state: VehicleState,
    seq: u64,
    path: PathBuf,
}

impl TelemetryReplayer {
    pub fn open(path: PathBuf) -> Result<Self, String> {
        let file = File::open(&path).map_err(|e| format!("open session: {e}"))?;
        let reader = BufReader::new(file);
        let mut lines = Vec::new();
        for line in reader.lines() {
            let line = line.map_err(|e| format!("read session: {e}"))?;
            if !line.trim().is_empty() {
                lines.push(line);
            }
        }
        if lines.is_empty() {
            return Err("Session file is empty".into());
        }
        Ok(Self {
            lines,
            index: 0,
            state: VehicleState::idle(),
            seq: 0,
            path,
        })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn total_lines(&self) -> usize {
        self.lines.len()
    }

    pub fn position(&self) -> usize {
        self.index
    }

    pub fn finished(&self) -> bool {
        self.index >= self.lines.len()
    }

    pub fn state(&self) -> &VehicleState {
        &self.state
    }

    pub fn seq(&self) -> u64 {
        self.seq
    }

    /// Advance one frame; returns false when the session ends.
    pub fn step(&mut self) -> bool {
        if self.index >= self.lines.len() {
            return false;
        }
        let line = &self.lines[self.index];
        self.index += 1;
        if let Ok(msg) = Message::parse_validated(line) {
            self.seq = msg.seq;
            if let Some(data) = msg.vss_data() {
                self.state.apply_vss_map(data);
            }
        }
        true
    }

    pub fn reset(&mut self) {
        self.index = 0;
        self.state = VehicleState::idle();
        self.seq = 0;
    }
}
