/// Severity of a diagnosis read.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReadingSeverity {
    Ok,
    Info,
    Warning,
    Critical,
}

impl ReadingSeverity {
    /// Uppercase label for the UI chip.
    pub fn label(self) -> &'static str {
        match self {
            Self::Ok => "OK",
            Self::Info => "INFO",
            Self::Warning => "WARNING",
            Self::Critical => "CRITICAL",
        }
    }

    pub(crate) fn rank(self) -> u8 {
        match self {
            Self::Ok => 0,
            Self::Info => 1,
            Self::Warning => 2,
            Self::Critical => 3,
        }
    }

    /// The more severe of the two.
    pub(crate) fn escalate(self, other: Self) -> Self {
        if other.rank() > self.rank() {
            other
        } else {
            self
        }
    }
}

/// Where the prose in a reading came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReadingSource {
    Model,
    RuleBased,
}

impl ReadingSource {
    /// Short label for the UI ("on-prem model" / "rule-based").
    pub fn label(self) -> &'static str {
        match self {
            Self::Model => "on-prem model",
            Self::RuleBased => "rule-based",
        }
    }
}

/// A plain-language read of the current diagnosis snapshot.
///
/// `severity` is always computed deterministically from the signals; only
/// `body` may come from a language model (see [`ReadingSource`]).
#[derive(Debug, Clone)]
pub struct DiagnosisReading {
    pub severity: ReadingSeverity,
    pub headline: String,
    pub body: String,
    pub source: ReadingSource,
}
