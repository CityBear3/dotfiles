use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TokenSums {
    pub input: u64,
    pub output: u64,
    pub cache_5m: u64,
    pub cache_1h: u64,
    pub cache_read: u64,
}

impl TokenSums {
    pub fn add(&mut self, o: &TokenSums) {
        self.input += o.input;
        self.output += o.output;
        self.cache_5m += o.cache_5m;
        self.cache_1h += o.cache_1h;
        self.cache_read += o.cache_read;
    }
}

/// month ("YYYY-MM", ローカル TZ) -> model id -> sums
pub type MonthModelSums = HashMap<String, HashMap<String, TokenSums>>;

#[derive(Debug, Default)]
pub struct ParsedDelta {
    pub by_month_model: MonthModelSums,
    pub new_seen: Vec<u64>,
    /// chunk 先頭からの消費バイト数。改行で終わらない末尾の未完行は含めない
    /// (statusline 実行中も Claude Code が追記し続けるため)。
    pub consumed: u64,
}

#[derive(Deserialize)]
struct Line {
    #[serde(rename = "type")]
    kind: Option<String>,
    timestamp: Option<String>,
    #[serde(rename = "requestId")]
    request_id: Option<String>,
    message: Option<Msg>,
}

#[derive(Deserialize)]
struct Msg {
    id: Option<String>,
    model: Option<String>,
    usage: Option<Usage>,
}

#[derive(Deserialize)]
struct Usage {
    #[serde(default)]
    input_tokens: u64,
    #[serde(default)]
    output_tokens: u64,
    #[serde(default)]
    cache_read_input_tokens: u64,
    #[serde(default)]
    cache_creation_input_tokens: u64,
    cache_creation: Option<CacheCreation>,
}

#[derive(Deserialize)]
struct CacheCreation {
    #[serde(default)]
    ephemeral_5m_input_tokens: u64,
    #[serde(default)]
    ephemeral_1h_input_tokens: u64,
}

pub fn dedup_hash(message_id: &str, request_id: &str) -> u64 {
    let mut h = DefaultHasher::new();
    message_id.hash(&mut h);
    request_id.hash(&mut h);
    h.finish()
}

/// offset 以降のバイト列を行単位でパースして集計する。
/// - type != "assistant"、model 無し / "<synthetic>"、usage 無しの行はスキップ
/// - (message.id, requestId) の重複はスキップ(1 API メッセージが content block
///   ごとに複数行書かれ、usage は同一のため初出のみカウントする)
pub fn parse_chunk(chunk: &[u8], seen: &HashSet<u64>) -> ParsedDelta {
    let mut delta = ParsedDelta::default();
    let mut chunk_seen: HashSet<u64> = HashSet::new();
    let mut pos = 0usize;
    while let Some(nl) = chunk[pos..].iter().position(|&b| b == b'\n') {
        let line = &chunk[pos..pos + nl];
        pos += nl + 1;
        process_line(line, seen, &mut chunk_seen, &mut delta);
    }
    delta.consumed = pos as u64;
    delta
}

fn process_line(
    line: &[u8],
    seen: &HashSet<u64>,
    chunk_seen: &mut HashSet<u64>,
    delta: &mut ParsedDelta,
) {
    let Ok(parsed) = serde_json::from_slice::<Line>(line) else {
        return;
    };
    if parsed.kind.as_deref() != Some("assistant") {
        return;
    }
    let Some(msg) = parsed.message else { return };
    let Some(model) = msg.model else { return };
    if model == "<synthetic>" {
        return;
    }
    let Some(usage) = msg.usage else { return };
    let id = msg.id.unwrap_or_default();
    let req = parsed.request_id.unwrap_or_default();
    let h = dedup_hash(&id, &req);
    if seen.contains(&h) || !chunk_seen.insert(h) {
        return;
    }
    delta.new_seen.push(h);

    let month = parsed
        .timestamp
        .as_deref()
        .and_then(local_month)
        .unwrap_or_else(|| "unknown".to_string());

    let (c5m, c1h) = match usage.cache_creation {
        Some(cc) => (cc.ephemeral_5m_input_tokens, cc.ephemeral_1h_input_tokens),
        // 古い transcript 形式: 内訳が無ければ全量を 5m 扱い
        None => (usage.cache_creation_input_tokens, 0),
    };
    let sums = TokenSums {
        input: usage.input_tokens,
        output: usage.output_tokens,
        cache_5m: c5m,
        cache_1h: c1h,
        cache_read: usage.cache_read_input_tokens,
    };
    delta
        .by_month_model
        .entry(month)
        .or_default()
        .entry(model)
        .or_default()
        .add(&sums);
}

/// RFC3339 の timestamp をローカルタイムゾーンの "YYYY-MM" へ。
fn local_month(ts: &str) -> Option<String> {
    use chrono::{DateTime, Local};
    let dt = DateTime::parse_from_rfc3339(ts).ok()?;
    Some(dt.with_timezone(&Local).format("%Y-%m").to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assistant_line(msg_id: &str, req_id: &str, model: &str, output: u64) -> String {
        let ts = chrono::Local::now().to_rfc3339();
        format!(
            r#"{{"type":"assistant","timestamp":"{ts}","requestId":"{req_id}","message":{{"id":"{msg_id}","model":"{model}","usage":{{"input_tokens":10,"output_tokens":{output},"cache_read_input_tokens":100,"cache_creation_input_tokens":30,"cache_creation":{{"ephemeral_5m_input_tokens":20,"ephemeral_1h_input_tokens":10}}}}}}}}"#
        )
    }

    fn current_month() -> String {
        chrono::Local::now().format("%Y-%m").to_string()
    }

    #[test]
    fn aggregates_by_month_and_model() {
        let chunk = format!(
            "{}\n{}\n",
            assistant_line("m1", "r1", "claude-opus-4-8", 200),
            assistant_line("m2", "r2", "claude-haiku-4-5", 300),
        );
        let d = parse_chunk(chunk.as_bytes(), &HashSet::new());
        let month = d.by_month_model.get(&current_month()).unwrap();
        assert_eq!(month.get("claude-opus-4-8").unwrap().output, 200);
        assert_eq!(month.get("claude-haiku-4-5").unwrap().output, 300);
        assert_eq!(month.get("claude-opus-4-8").unwrap().cache_5m, 20);
        assert_eq!(month.get("claude-opus-4-8").unwrap().cache_1h, 10);
        assert_eq!(d.new_seen.len(), 2);
        assert_eq!(d.consumed, chunk.len() as u64);
    }

    #[test]
    fn dedups_repeated_message_id() {
        let line = assistant_line("m1", "r1", "claude-opus-4-8", 200);
        let chunk = format!("{line}\n{line}\n{line}\n");
        let d = parse_chunk(chunk.as_bytes(), &HashSet::new());
        let month = d.by_month_model.get(&current_month()).unwrap();
        assert_eq!(month.get("claude-opus-4-8").unwrap().output, 200);
        assert_eq!(d.new_seen.len(), 1);
    }

    #[test]
    fn respects_previously_seen_set() {
        let line = assistant_line("m1", "r1", "claude-opus-4-8", 200);
        let seen: HashSet<u64> = [dedup_hash("m1", "r1")].into_iter().collect();
        let d = parse_chunk(format!("{line}\n").as_bytes(), &seen);
        assert!(d.by_month_model.is_empty());
        assert!(d.new_seen.is_empty());
    }

    #[test]
    fn skips_non_assistant_synthetic_and_broken_lines() {
        let ts = chrono::Local::now().to_rfc3339();
        let user_line = r#"{"type":"user","message":{}}"#;
        let synthetic_line = format!(
            r#"{{"type":"assistant","timestamp":"{ts}","message":{{"id":"s1","model":"<synthetic>","usage":{{"input_tokens":5}}}}}}"#
        );
        let good_line = assistant_line("m1", "r1", "claude-opus-4-8", 200);
        let chunk = format!("{user_line}\n{synthetic_line}\nnot json at all\n{good_line}\n");
        let d = parse_chunk(chunk.as_bytes(), &HashSet::new());
        let month = d.by_month_model.get(&current_month()).unwrap();
        assert_eq!(month.len(), 1);
        assert_eq!(d.consumed, chunk.len() as u64);
    }

    #[test]
    fn incomplete_trailing_line_is_not_consumed() {
        let full = assistant_line("m1", "r1", "claude-opus-4-8", 200);
        let partial = r#"{"type":"assistant","time"#;
        let chunk = format!("{full}\n{partial}");
        let d = parse_chunk(chunk.as_bytes(), &HashSet::new());
        assert_eq!(d.consumed, (full.len() + 1) as u64);
        assert_eq!(d.new_seen.len(), 1);
    }

    #[test]
    fn legacy_usage_without_cache_breakdown_counts_as_5m() {
        let ts = chrono::Local::now().to_rfc3339();
        let line = format!(
            r#"{{"type":"assistant","timestamp":"{ts}","requestId":"r1","message":{{"id":"m1","model":"claude-opus-4-8","usage":{{"input_tokens":1,"output_tokens":2,"cache_read_input_tokens":3,"cache_creation_input_tokens":40}}}}}}"#
        );
        let d = parse_chunk(format!("{line}\n").as_bytes(), &HashSet::new());
        let sums = d
            .by_month_model
            .get(&current_month())
            .unwrap()
            .get("claude-opus-4-8")
            .unwrap();
        assert_eq!(sums.cache_5m, 40);
        assert_eq!(sums.cache_1h, 0);
    }
}
