use serde::Deserialize;

/// Claude Code が statusline コマンドの stdin に渡す JSON。
/// 必要なフィールドのみ定義する(未知フィールドは serde が無視する)。
#[derive(Debug, Default, Deserialize)]
pub struct StatusInput {
    pub transcript_path: Option<String>,
    #[serde(default)]
    pub model: Model,
    #[serde(default)]
    pub workspace: Workspace,
    pub cost: Option<Cost>,
    pub context_window: Option<ContextWindow>,
    pub rate_limits: Option<RateLimits>,
}

#[derive(Debug, Default, Deserialize)]
pub struct Model {
    pub display_name: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
pub struct Workspace {
    pub current_dir: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Cost {
    pub total_cost_usd: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct ContextWindow {
    pub remaining_percentage: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct RateLimits {
    pub five_hour: Option<RateLimitWindow>,
    pub seven_day: Option<RateLimitWindow>,
}

#[derive(Debug, Deserialize)]
pub struct RateLimitWindow {
    pub used_percentage: Option<f64>,
    /// unix epoch 秒
    pub resets_at: Option<f64>,
}

impl StatusInput {
    pub fn parse(json: &str) -> Option<Self> {
        serde_json::from_str(json).ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FULL: &str = r#"{
        "session_id": "abc",
        "transcript_path": "/tmp/t.jsonl",
        "model": {"id": "claude-fable-5", "display_name": "Fable 5"},
        "workspace": {"current_dir": "/Users/x/dotfiles/claude", "project_dir": "/Users/x/dotfiles"},
        "cost": {"total_cost_usd": 1.23, "total_duration_ms": 100},
        "context_window": {"context_window_size": 1000000, "used_percentage": 28.0, "remaining_percentage": 72.0},
        "rate_limits": {
            "five_hour": {"used_percentage": 41.0, "resets_at": 1751700000},
            "seven_day": {"used_percentage": 23.0, "resets_at": 1751900000}
        }
    }"#;

    #[test]
    fn parses_full_payload() {
        let i = StatusInput::parse(FULL).unwrap();
        assert_eq!(i.transcript_path.as_deref(), Some("/tmp/t.jsonl"));
        assert_eq!(i.model.display_name.as_deref(), Some("Fable 5"));
        assert_eq!(i.cost.unwrap().total_cost_usd, Some(1.23));
        assert_eq!(i.context_window.unwrap().remaining_percentage, Some(72.0));
        let rl = i.rate_limits.unwrap();
        assert_eq!(rl.five_hour.unwrap().used_percentage, Some(41.0));
        assert_eq!(rl.seven_day.unwrap().resets_at, Some(1751900000.0));
    }

    #[test]
    fn parses_minimal_payload() {
        let i = StatusInput::parse(r#"{"session_id":"x"}"#).unwrap();
        assert!(i.transcript_path.is_none());
        assert!(i.rate_limits.is_none());
        assert!(i.context_window.is_none());
    }

    #[test]
    fn broken_json_returns_none() {
        assert!(StatusInput::parse("not json").is_none());
    }
}
