/// On-prem inference endpoint configuration (OpenAI-compatible chat API).
///
/// All vehicle / CAN data stays in-shop: nothing is sent anywhere unless
/// `SIGMA_AI_BASE_URL` points at a host you run. With no base URL the advisor
/// stays fully offline (rule-based).
#[derive(Debug, Clone, Default)]
pub struct AiConfig {
    /// Base URL of an OpenAI-compatible server, e.g. `http://localhost:11434/v1`.
    /// Empty disables the model.
    pub base_url: String,
    /// Model name to request (server-defined, e.g. `qwen2.5:7b-instruct`).
    pub model: String,
    /// Optional bearer token; many local servers need none.
    pub api_key: Option<String>,
    /// Per-request timeout in seconds.
    pub timeout_secs: u64,
}

impl AiConfig {
    /// Read the endpoint from the environment. Defaults keep the model off
    /// (empty base URL) so on-prem inference is strictly opt-in.
    pub fn from_env() -> Self {
        let base_url = std::env::var("SIGMA_AI_BASE_URL")
            .unwrap_or_default()
            .trim_end_matches('/')
            .to_owned();
        let model =
            std::env::var("SIGMA_AI_MODEL").unwrap_or_else(|_| "qwen2.5:7b-instruct".into());
        let api_key = std::env::var("SIGMA_AI_API_KEY")
            .ok()
            .filter(|s| !s.is_empty());
        let timeout_secs = std::env::var("SIGMA_AI_TIMEOUT_SECS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(30);
        Self {
            base_url,
            model,
            api_key,
            timeout_secs,
        }
    }

    /// Whether a model endpoint is configured.
    pub fn enabled(&self) -> bool {
        !self.base_url.is_empty()
    }

    /// Chat-completions endpoint URL.
    pub fn chat_url(&self) -> String {
        format!("{}/chat/completions", self.base_url)
    }

    /// Short human label for the configured backend.
    pub fn endpoint_label(&self) -> String {
        if self.enabled() {
            format!("{} @ {}", self.model, self.base_url)
        } else {
            "rule-based (offline)".into()
        }
    }
}
