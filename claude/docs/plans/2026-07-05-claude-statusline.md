# claude-statusline (Rust) Implementation Plan

> **Execution:** Use `/execute-plan` to dispatch this plan to agent-teams. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Claude Code の statusline に context 残量・5h/週次使用量・セッションコスト(モデル別)・月次トータルコストを表示する Rust バイナリ `claude-statusline` を実装し、install.sh で配布する。

**Architecture:** stdin JSON(Claude Code が渡す)から context/rate_limits/セッション総コストを読み、transcript JSONL(`~/.claude/projects/**/*.jsonl`)の増分パース(ファイル別オフセット + 月別・モデル別トークン集計の永続キャッシュ)でモデル別セッションコストと月次トータルを自前計算する。単価はバイナリ埋め込み + `~/.claude/statusline.toml` で上書き可。出力は ANSI カラー付き 2 行。lib + bin 構成(`src/lib.rs` がモジュールを公開、`src/main.rs` は薄いエントリ)。

**Tech Stack:** Rust 2021 (cargo 1.96)、serde / serde_json / toml / chrono。外部プロセス・ネットワーク依存なし。

**Working directory:** リポジトリルート(`dotfiles/`)からの相対で `claude/statusline/`(Task 1 で作成)。build/test コマンドはすべてそこで実行する。Task 8 のみ `claude/` 直下を編集する。
**Branch:** `feature/claude-statusline`
**Baseline before Task 1:** リポジトリは clean(`git status` で変更なし)。`cargo --version` が 1.96 系で動くこと。Rust コードはまだ存在しない。

**Per-task verification command** (mandatory before each commit):
```sh
cd claude/statusline && cargo fmt && cargo test --quiet && cargo clippy --all-targets -- -D warnings && cargo fmt -- --check
```

---

## 設計上の重要な前提(全タスク共通の背景)

- **stdin JSON**(確認済みスキーマ、必要フィールドのみ):
  `transcript_path`, `model.display_name`, `workspace.current_dir`, `cost.total_cost_usd`,
  `context_window.remaining_percentage`, `rate_limits.five_hour|seven_day.{used_percentage, resets_at(unix秒)}`。
  `rate_limits` は Pro/Max サブスク時のみ・初回 API 応答後のみ存在。`context_window` の percentage はセッション開始直後 null。
- **transcript JSONL** の assistant 行(実データで確認済み):
  ```json
  {"type":"assistant","timestamp":"2026-04-21T18:09:28.470Z","requestId":"req_..",
   "message":{"id":"msg_..","model":"claude-opus-4-7",
     "usage":{"input_tokens":6,"output_tokens":171,"cache_read_input_tokens":0,
       "cache_creation_input_tokens":29255,
       "cache_creation":{"ephemeral_5m_input_tokens":0,"ephemeral_1h_input_tokens":29255}}}}
  ```
  - `costUSD` は null → トークン数 × 単価で自前計算する。
  - **同一 `message.id` の行が複数回出現する**(1 メッセージが content block ごとに複数行書かれる。実測: 5889 行中ユニーク 2797、非隣接の重複あり)。usage は同一なので `(message.id, requestId)` の初出のみカウントする。
  - `model` が `"<synthetic>"` の行はスキップ。
- **単価**(公式ドキュメント 2026-06-24 時点、$/MTok)。cache write 5m = input×1.25、1h = input×2.0、cache read = input×0.1:
  | model prefix | input | output |
  |---|---|---|
  | claude-fable-5 / claude-mythos-5 | 10 | 50 |
  | claude-opus-4-8 / -4-7 / -4-6 / -4-5 | 5 | 25 |
  | claude-sonnet-5 / -4-6 / -4-5 | 3 | 15 |
  | claude-haiku-4-5 | 1 | 5 |
  Opus 4.7 以降・Fable 5 は 1M コンテキストでも標準価格(long-context premium なし)のため 2 段単価は実装しない。価格改定・intro 価格(Sonnet 5 は 2026-08-31 まで $2/$10)は TOML 上書きで対応する。
- **性能予算**: statusline はイベント駆動 + 300ms デバウンス。増分キャッシュにより通常起動は数十 ms、初回のみ当月分フルスキャン。
- **確定レイアウト**(ANSI カラーは dark-daltonized 向けに青/オレンジ系):
  ```
  dotfiles ⎇ main  Fable 5  ctx ███████░░░ 72% free
  5h 41% ↻21:00  wk 23% ↻Mon 09:00  S $1.23 (fable-5 $0.90 | haiku-4-5 $0.33)  M $45.6
  ```
  データ欠落時: ctx → `ctx –`、rate_limits 無し → セグメント自体を省略、単価不明モデル → `名前 ?`(月次には `+?` マーカー)。

---

### Task 1: Crate scaffolding + stdin JSON model (`input.rs`)

**Why:** すべてのタスクの土台となる crate 構成(lib + bin)を作り、Claude Code から渡される stdin JSON を型に落とす。

**Behavior change:** yes(新規 crate・stdin パース機能)
**Discipline:** TDD — fixture JSON のパーステストを先に書く。

**Files:**
- Create: `claude/statusline/Cargo.toml`
- Create: `claude/statusline/.gitignore`
- Create: `claude/statusline/src/lib.rs`
- Create: `claude/statusline/src/main.rs`
- Create: `claude/statusline/src/input.rs`

### Steps

- [ ] **Step 1: crate を作成する**

```sh
mkdir -p claude/statusline/src
```

`claude/statusline/Cargo.toml`:
```toml
[package]
name = "claude-statusline"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
toml = "1"
chrono = "0.4"
```

`claude/statusline/.gitignore`:
```
/target
```

`claude/statusline/src/lib.rs`(このタスク時点):
```rust
pub mod input;
```

`claude/statusline/src/main.rs`(暫定。Task 7 で完成させる):
```rust
fn main() {
    use std::io::Read;
    let mut buf = String::new();
    let _ = std::io::stdin().read_to_string(&mut buf);
    let _ = claude_statusline::input::StatusInput::parse(&buf);
    println!("claude-statusline: (wip)");
}
```

- [ ] **Step 2: 失敗するテストを含む `input.rs` を書く(red)**

`claude/statusline/src/input.rs`:
```rust
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
        assert_eq!(
            i.context_window.unwrap().remaining_percentage,
            Some(72.0)
        );
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
```

- [ ] **Step 3: red → green を確認する**

Run: `cd claude/statusline && cargo test --quiet`
Expected: 3 テストすべて PASS(コードとテストを同時に書くため、先に `parse` の実装を `None` 固定にして FAIL を確認してから本実装に置き換えてもよい)。

- [ ] **Step 4: Verify**

```sh
cd claude/statusline && cargo fmt && cargo test --quiet && cargo clippy --all-targets -- -D warnings && cargo fmt -- --check
```

Expected: 3 tests pass; clippy/fmt clean。

- [ ] **Step 5: Commit**

```sh
git add claude/statusline
git commit -m "$(cat <<'EOF'
Add claude-statusline crate with stdin JSON input model

Claude Code statusline 用 Rust バイナリの土台。stdin で渡される
status JSON から必要フィールドのみを serde で取り出す。
EOF
)"
```

---

### Task 2: 単価表 (`pricing.rs`)

**Why:** transcript にはトークン数しか無いため、モデル別単価($/MTok、cache write 5m/1h・cache read 含む)をバイナリに埋め込み、`~/.claude/statusline.toml` で上書き可能にする。コスト計算関数もここに置く。

**Behavior change:** yes(単価表とコスト計算の新規実装)
**Discipline:** TDD

**Files:**
- Create: `claude/statusline/src/pricing.rs`
- Modify: `claude/statusline/src/lib.rs`(`pub mod pricing;` 追加)

**注意:** `pricing.rs` は Task 3 で作る `transcript::TokenSums` を参照する。タスク順序の都合上、このタスクでは `TokenSums` を先に `transcript.rs` の**最小版**として作成する(Task 3 がそれを拡張する)。

### Steps

- [ ] **Step 1: `transcript.rs` の最小版(TokenSums のみ)を作る**

`claude/statusline/src/transcript.rs`:
```rust
use serde::{Deserialize, Serialize};

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
```

`claude/statusline/src/lib.rs` を更新:
```rust
pub mod input;
pub mod pricing;
pub mod transcript;
```

- [ ] **Step 2: 失敗するテストを含む `pricing.rs` を書く(red → green)**

`claude/statusline/src/pricing.rs`:
```rust
use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;

use crate::transcript::TokenSums;

/// $/MTok
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct ModelPrice {
    pub input: f64,
    pub output: f64,
    pub cache_write_5m: f64,
    pub cache_write_1h: f64,
    pub cache_read: f64,
}

/// 埋め込み価格表(2026-07 時点の公表価格、$/MTok)。
/// cache write 5m = input x1.25 / 1h = x2.0 / cache read = x0.1。
/// prefix 一致で引くため、日付サフィックス付き ID (claude-haiku-4-5-20251001) も拾える。
/// 価格改定は ~/.claude/statusline.toml の [pricing."<model-id>"] で上書きする。
const EMBEDDED: &[(&str, ModelPrice)] = &[
    ("claude-fable-5", OPUS_X2),
    ("claude-mythos-5", OPUS_X2),
    ("claude-opus-4-8", OPUS),
    ("claude-opus-4-7", OPUS),
    ("claude-opus-4-6", OPUS),
    ("claude-opus-4-5", OPUS),
    ("claude-sonnet-5", SONNET),
    ("claude-sonnet-4-6", SONNET),
    ("claude-sonnet-4-5", SONNET),
    ("claude-haiku-4-5", HAIKU),
];

const OPUS_X2: ModelPrice = ModelPrice {
    input: 10.0,
    output: 50.0,
    cache_write_5m: 12.5,
    cache_write_1h: 20.0,
    cache_read: 1.0,
};
const OPUS: ModelPrice = ModelPrice {
    input: 5.0,
    output: 25.0,
    cache_write_5m: 6.25,
    cache_write_1h: 10.0,
    cache_read: 0.5,
};
const SONNET: ModelPrice = ModelPrice {
    input: 3.0,
    output: 15.0,
    cache_write_5m: 3.75,
    cache_write_1h: 6.0,
    cache_read: 0.3,
};
const HAIKU: ModelPrice = ModelPrice {
    input: 1.0,
    output: 5.0,
    cache_write_5m: 1.25,
    cache_write_1h: 2.0,
    cache_read: 0.1,
};

pub struct PricingTable {
    entries: Vec<(String, ModelPrice)>,
}

#[derive(Deserialize)]
struct OverrideFile {
    #[serde(default)]
    pricing: HashMap<String, ModelPrice>,
}

impl PricingTable {
    pub fn embedded() -> Self {
        Self {
            entries: EMBEDDED.iter().map(|(k, v)| (k.to_string(), *v)).collect(),
        }
    }

    /// TOML(全 5 フィールド必須)で追加・上書き。読めない/壊れている場合は無視。
    pub fn load_overrides(&mut self, path: &Path) {
        let Ok(text) = std::fs::read_to_string(path) else {
            return;
        };
        let Ok(file) = toml::from_str::<OverrideFile>(&text) else {
            return;
        };
        for (id, price) in file.pricing {
            if let Some(e) = self.entries.iter_mut().find(|(k, _)| *k == id) {
                e.1 = price;
            } else {
                self.entries.push((id, price));
            }
        }
    }

    /// 最長 prefix 一致。
    pub fn lookup(&self, model_id: &str) -> Option<ModelPrice> {
        self.entries
            .iter()
            .filter(|(k, _)| model_id.starts_with(k.as_str()))
            .max_by_key(|(k, _)| k.len())
            .map(|(_, p)| *p)
    }
}

const MTOK: f64 = 1_000_000.0;

pub fn cost_usd(t: &TokenSums, p: &ModelPrice) -> f64 {
    (t.input as f64 * p.input
        + t.output as f64 * p.output
        + t.cache_5m as f64 * p.cache_write_5m
        + t.cache_1h as f64 * p.cache_write_1h
        + t.cache_read as f64 * p.cache_read)
        / MTOK
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lookup_exact_and_prefix() {
        let t = PricingTable::embedded();
        assert_eq!(t.lookup("claude-fable-5").unwrap().input, 10.0);
        // 日付サフィックス付きは prefix 一致
        assert_eq!(t.lookup("claude-haiku-4-5-20251001").unwrap().input, 1.0);
        assert!(t.lookup("gpt-4o").is_none());
    }

    #[test]
    fn cost_computation() {
        let sums = TokenSums {
            input: 1_000_000,
            output: 1_000_000,
            cache_5m: 1_000_000,
            cache_1h: 1_000_000,
            cache_read: 1_000_000,
        };
        let p = PricingTable::embedded().lookup("claude-opus-4-8").unwrap();
        // 5 + 25 + 6.25 + 10 + 0.5 = 46.75
        assert!((cost_usd(&sums, &p) - 46.75).abs() < 1e-9);
    }

    #[test]
    fn overrides_replace_and_extend() {
        let dir = std::env::temp_dir();
        let path = dir.join(format!("cs-pricing-{}.toml", std::process::id()));
        std::fs::write(
            &path,
            r#"
[pricing."claude-opus-4-8"]
input = 1.0
output = 2.0
cache_write_5m = 1.25
cache_write_1h = 2.0
cache_read = 0.1

[pricing."my-custom-model"]
input = 7.0
output = 8.0
cache_write_5m = 8.75
cache_write_1h = 14.0
cache_read = 0.7
"#,
        )
        .unwrap();
        let mut t = PricingTable::embedded();
        t.load_overrides(&path);
        assert_eq!(t.lookup("claude-opus-4-8").unwrap().input, 1.0);
        assert_eq!(t.lookup("my-custom-model-v2").unwrap().input, 7.0);
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn missing_or_broken_override_is_ignored() {
        let mut t = PricingTable::embedded();
        t.load_overrides(Path::new("/nonexistent/statusline.toml"));
        assert_eq!(t.lookup("claude-opus-4-8").unwrap().input, 5.0);

        let path =
            std::env::temp_dir().join(format!("cs-broken-{}.toml", std::process::id()));
        std::fs::write(&path, "not [ valid toml").unwrap();
        t.load_overrides(&path);
        assert_eq!(t.lookup("claude-opus-4-8").unwrap().input, 5.0);
        let _ = std::fs::remove_file(&path);
    }
}
```

- [ ] **Step 3: Verify**

```sh
cd claude/statusline && cargo fmt && cargo test --quiet && cargo clippy --all-targets -- -D warnings && cargo fmt -- --check
```

Expected: 累計 7 tests pass; clippy/fmt clean。

- [ ] **Step 4: Commit**

```sh
git add claude/statusline
git commit -m "$(cat <<'EOF'
Add embedded pricing table with TOML overrides

モデル別単価 ($/MTok、cache write 5m/1h・read 含む) をバイナリに
埋め込み、~/.claude/statusline.toml の [pricing."<model-id>"] で
追加・上書き可能にする。モデル ID は最長 prefix 一致で解決する。
EOF
)"
```

---

### Task 3: transcript JSONL の増分パース (`transcript.rs`)

**Why:** モデル別コストの原データは transcript JSONL のみ。追記専用ファイルをオフセットから差分パースし、重複行(同一 message.id が複数回出現する)を排除して月別・モデル別にトークンを集計する。

**Behavior change:** yes(JSONL パース・集計の新規実装)
**Discipline:** TDD

**Files:**
- Modify: `claude/statusline/src/transcript.rs`(Task 2 の最小版を拡張)

### Steps

- [ ] **Step 1: 失敗するテストを含む完全版 `transcript.rs` を書く(red → green)**

`claude/statusline/src/transcript.rs` 全体を以下に置き換える:
```rust
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
```

- [ ] **Step 2: Verify**

```sh
cd claude/statusline && cargo fmt && cargo test --quiet && cargo clippy --all-targets -- -D warnings && cargo fmt -- --check
```

Expected: 累計 13 tests pass; clippy/fmt clean。

- [ ] **Step 3: Commit**

```sh
git add claude/statusline
git commit -m "$(cat <<'EOF'
Add incremental transcript JSONL parser with dedup

追記専用 JSONL をオフセットから行単位でパースし、(message.id,
requestId) の初出のみを月別・モデル別に集計する。1 API メッセージが
content block ごとに複数行書かれるため重複排除が必須(排除しないと
コストが約 2 倍になる)。未完の末尾行は consumed に含めず持ち越す。
EOF
)"
```

---

### Task 4: git ブランチ検出 (`git.rs`)

**Why:** 1 行目にブランチ名を表示する。`git` サブプロセス起動(数十 ms)を避け、`.git/HEAD` を直接読む。worktree(`.git` がファイルの場合)にも対応する。

**Behavior change:** yes(新規機能)
**Discipline:** TDD

**Files:**
- Create: `claude/statusline/src/git.rs`
- Modify: `claude/statusline/src/lib.rs`(`pub mod git;` 追加)

### Steps

- [ ] **Step 1: 失敗するテストを含む `git.rs` を書く(red → green)**

`claude/statusline/src/git.rs`:
```rust
use std::fs;
use std::path::Path;

/// start から親方向に .git を探し、ブランチ名を返す。
/// detached HEAD は短縮 SHA(7 桁)、リポジトリ外は None。
/// worktree(.git が "gitdir: <path>" ファイル)にも対応する。
pub fn current_branch(start: &Path) -> Option<String> {
    let mut dir = Some(start);
    while let Some(d) = dir {
        let dotgit = d.join(".git");
        if dotgit.is_dir() {
            return read_head(&dotgit);
        }
        if dotgit.is_file() {
            let text = fs::read_to_string(&dotgit).ok()?;
            let gitdir = text.strip_prefix("gitdir:")?.trim().to_string();
            return read_head(Path::new(&gitdir));
        }
        dir = d.parent();
    }
    None
}

fn read_head(gitdir: &Path) -> Option<String> {
    let head = fs::read_to_string(gitdir.join("HEAD")).ok()?;
    let head = head.trim();
    if let Some(r) = head.strip_prefix("ref: refs/heads/") {
        return Some(r.to_string());
    }
    Some(head.chars().take(7).collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn tmp(name: &str) -> PathBuf {
        let p = std::env::temp_dir().join(format!("cs-git-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&p);
        fs::create_dir_all(&p).unwrap();
        p
    }

    #[test]
    fn reads_branch_from_head() {
        let root = tmp("branch");
        fs::create_dir_all(root.join(".git")).unwrap();
        fs::write(root.join(".git/HEAD"), "ref: refs/heads/main\n").unwrap();
        assert_eq!(current_branch(&root).as_deref(), Some("main"));
    }

    #[test]
    fn walks_up_to_repo_root() {
        let root = tmp("nested");
        fs::create_dir_all(root.join(".git")).unwrap();
        fs::write(root.join(".git/HEAD"), "ref: refs/heads/feature/x\n").unwrap();
        let nested = root.join("a/b/c");
        fs::create_dir_all(&nested).unwrap();
        assert_eq!(current_branch(&nested).as_deref(), Some("feature/x"));
    }

    #[test]
    fn detached_head_returns_short_sha() {
        let root = tmp("detached");
        fs::create_dir_all(root.join(".git")).unwrap();
        fs::write(
            root.join(".git/HEAD"),
            "0123456789abcdef0123456789abcdef01234567\n",
        )
        .unwrap();
        assert_eq!(current_branch(&root).as_deref(), Some("0123456"));
    }

    #[test]
    fn worktree_gitdir_file() {
        let real = tmp("wt-real");
        fs::write(real.join("HEAD"), "ref: refs/heads/wt-branch\n").unwrap();
        let wt = tmp("wt-link");
        fs::write(wt.join(".git"), format!("gitdir: {}\n", real.display())).unwrap();
        assert_eq!(current_branch(&wt).as_deref(), Some("wt-branch"));
    }
}
```

`claude/statusline/src/lib.rs` に `pub mod git;` を追加(アルファベット順を維持):
```rust
pub mod git;
pub mod input;
pub mod pricing;
pub mod transcript;
```

- [ ] **Step 2: Verify**

```sh
cd claude/statusline && cargo fmt && cargo test --quiet && cargo clippy --all-targets -- -D warnings && cargo fmt -- --check
```

Expected: 累計 17 tests pass; clippy/fmt clean。

- [ ] **Step 3: Commit**

```sh
git add claude/statusline
git commit -m "$(cat <<'EOF'
Add git branch detection via .git/HEAD

サブプロセス起動を避けるため HEAD ファイルを直接読む。
worktree (gitdir ファイル) と detached HEAD に対応。
EOF
)"
```

---

### Task 5: 増分キャッシュ (`cache.rs`)

**Why:** 300ms 予算内で月次集計を行うため、ファイル別の読込オフセット・重複排除セット・月別モデル別集計を `$XDG_CACHE_HOME/claude-statusline/cache.json`(未設定時 `~/.cache/...`)に永続化し、変更のあったファイルだけ差分パースする。

**Behavior change:** yes(キャッシュ機構の新規実装)
**Discipline:** TDD

**Files:**
- Create: `claude/statusline/src/cache.rs`
- Modify: `claude/statusline/src/lib.rs`(`pub mod cache;` 追加)

### Steps

- [ ] **Step 1: 失敗するテストを含む `cache.rs` を書く(red → green)**

`claude/statusline/src/cache.rs`:
```rust
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use crate::transcript::{self, MonthModelSums, TokenSums};

const CACHE_VERSION: u32 = 1;

#[derive(Serialize, Deserialize, Default)]
pub struct Cache {
    pub version: u32,
    pub files: HashMap<String, FileEntry>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct FileEntry {
    pub offset: u64,
    /// dedup_hash(message.id, requestId) の集合
    pub seen: Vec<u64>,
    pub months: MonthModelSums,
}

/// 読めない・壊れている・バージョン不一致の場合は空キャッシュから再構築する。
pub fn load(path: &Path) -> Cache {
    let cache = fs::read_to_string(path)
        .ok()
        .and_then(|s| serde_json::from_str::<Cache>(&s).ok())
        .unwrap_or_default();
    if cache.version == CACHE_VERSION {
        cache
    } else {
        Cache {
            version: CACHE_VERSION,
            files: HashMap::new(),
        }
    }
}

/// PID 付き temp file + rename で原子的に保存する。複数セッションの
/// statusline が並行起動しても tmp が衝突せず、読み手が書きかけの
/// cache.json を見ることもない(rename は POSIX で原子的)。
pub fn save(path: &Path, cache: &Cache) {
    if let Some(dir) = path.parent() {
        let _ = fs::create_dir_all(dir);
    }
    let Ok(json) = serde_json::to_string(cache) else {
        return;
    };
    let tmp = path.with_extension(format!("json.{}.tmp", std::process::id()));
    if fs::write(&tmp, json).is_ok() {
        let _ = fs::rename(&tmp, path);
    }
}

/// projects_dir 以下で mtime >= cutoff の JSONL と transcript_path を
/// 増分パースして cache を更新する。cutoff は「当月 1 日 0:00 − マージン」。
pub fn update(
    cache: &mut Cache,
    projects_dir: &Path,
    transcript_path: Option<&Path>,
    cutoff: SystemTime,
) {
    let mut candidates: Vec<PathBuf> = list_recent_jsonl(projects_dir, cutoff);
    if let Some(t) = transcript_path {
        if !candidates.iter().any(|c| c == t) {
            candidates.push(t.to_path_buf());
        }
    }

    for path in &candidates {
        let key = path.to_string_lossy().to_string();
        let Ok(meta) = fs::metadata(path) else { continue };
        let size = meta.len();
        let entry = cache.files.entry(key).or_default();
        if size < entry.offset {
            // truncate / 差し替え → 全再走査
            *entry = FileEntry::default();
        }
        if size == entry.offset {
            continue;
        }
        let Ok(mut f) = fs::File::open(path) else { continue };
        if f.seek(SeekFrom::Start(entry.offset)).is_err() {
            continue;
        }
        let mut buf = Vec::with_capacity((size - entry.offset) as usize);
        if f.read_to_end(&mut buf).is_err() {
            continue;
        }
        let seen: HashSet<u64> = entry.seen.iter().copied().collect();
        let delta = transcript::parse_chunk(&buf, &seen);
        entry.offset += delta.consumed;
        entry.seen.extend(delta.new_seen);
        for (month, models) in delta.by_month_model {
            let bucket = entry.months.entry(month).or_default();
            for (model, sums) in models {
                bucket.entry(model).or_default().add(&sums);
            }
        }
    }

    // 当月に関与しないエントリを落とす。cutoff より古い mtime のファイルは
    // 当月のエントリを含み得ない(最終書込が当月開始前)ため安全に破棄できる。
    let keep: HashSet<String> = candidates
        .iter()
        .map(|p| p.to_string_lossy().to_string())
        .collect();
    cache.files.retain(|k, _| keep.contains(k));
}

/// 当月合計(全ファイル、モデル別)。
pub fn month_model_sums(cache: &Cache, month: &str) -> HashMap<String, TokenSums> {
    let mut out: HashMap<String, TokenSums> = HashMap::new();
    for entry in cache.files.values() {
        if let Some(models) = entry.months.get(month) {
            for (model, sums) in models {
                out.entry(model.clone()).or_default().add(sums);
            }
        }
    }
    out
}

/// 1 ファイル(= 1 セッション)の全期間合計(モデル別)。
pub fn file_model_sums(cache: &Cache, path: &str) -> HashMap<String, TokenSums> {
    let mut out: HashMap<String, TokenSums> = HashMap::new();
    if let Some(entry) = cache.files.get(path) {
        for models in entry.months.values() {
            for (model, sums) in models {
                out.entry(model.clone()).or_default().add(sums);
            }
        }
    }
    out
}

fn list_recent_jsonl(projects_dir: &Path, cutoff: SystemTime) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let Ok(projects) = fs::read_dir(projects_dir) else {
        return out;
    };
    for proj in projects.flatten() {
        let Ok(sessions) = fs::read_dir(proj.path()) else {
            continue;
        };
        for f in sessions.flatten() {
            let path = f.path();
            if path.extension().and_then(|e| e.to_str()) != Some("jsonl") {
                continue;
            }
            let Ok(meta) = f.metadata() else { continue };
            let Ok(mtime) = meta.modified() else { continue };
            if mtime >= cutoff {
                out.push(path);
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, UNIX_EPOCH};

    fn tmp(name: &str) -> PathBuf {
        let p = std::env::temp_dir().join(format!("cs-cache-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&p);
        fs::create_dir_all(&p).unwrap();
        p
    }

    fn assistant_line(msg_id: &str, model: &str, output: u64) -> String {
        let ts = chrono::Local::now().to_rfc3339();
        format!(
            r#"{{"type":"assistant","timestamp":"{ts}","requestId":"req-{msg_id}","message":{{"id":"{msg_id}","model":"{model}","usage":{{"input_tokens":10,"output_tokens":{output},"cache_read_input_tokens":0,"cache_creation_input_tokens":0}}}}}}"#
        )
    }

    fn current_month() -> String {
        chrono::Local::now().format("%Y-%m").to_string()
    }

    #[test]
    fn save_load_roundtrip_and_version_check() {
        let dir = tmp("roundtrip");
        let path = dir.join("cache.json");
        let mut c = load(&path);
        assert_eq!(c.version, 1);
        c.files.insert(
            "/x.jsonl".into(),
            FileEntry {
                offset: 42,
                seen: vec![1, 2],
                months: MonthModelSums::new(),
            },
        );
        save(&path, &c);
        let c2 = load(&path);
        assert_eq!(c2.files.get("/x.jsonl").unwrap().offset, 42);

        // バージョン不一致 → 空から再構築
        fs::write(&path, r#"{"version":99,"files":{}}"#).unwrap();
        assert!(load(&path).files.is_empty());
    }

    #[test]
    fn update_parses_incrementally() {
        let home = tmp("incr");
        let proj = home.join("projects/p1");
        fs::create_dir_all(&proj).unwrap();
        let file = proj.join("sess.jsonl");
        fs::write(&file, format!("{}\n", assistant_line("m1", "claude-opus-4-8", 100))).unwrap();

        let mut cache = load(&home.join("cache.json"));
        update(&mut cache, &home.join("projects"), None, UNIX_EPOCH);
        let sums = month_model_sums(&cache, &current_month());
        assert_eq!(sums.get("claude-opus-4-8").unwrap().output, 100);

        // 追記 → 差分のみ反映
        let mut content = fs::read_to_string(&file).unwrap();
        content.push_str(&format!("{}\n", assistant_line("m2", "claude-opus-4-8", 50)));
        fs::write(&file, content).unwrap();
        update(&mut cache, &home.join("projects"), None, UNIX_EPOCH);
        let sums = month_model_sums(&cache, &current_month());
        assert_eq!(sums.get("claude-opus-4-8").unwrap().output, 150);

        // 変更なし → 集計は不変
        update(&mut cache, &home.join("projects"), None, UNIX_EPOCH);
        let sums = month_model_sums(&cache, &current_month());
        assert_eq!(sums.get("claude-opus-4-8").unwrap().output, 150);
    }

    #[test]
    fn dedup_survives_across_updates() {
        let home = tmp("dedup");
        let proj = home.join("projects/p1");
        fs::create_dir_all(&proj).unwrap();
        let file = proj.join("sess.jsonl");
        let line = assistant_line("m1", "claude-opus-4-8", 100);
        fs::write(&file, format!("{line}\n")).unwrap();

        let mut cache = load(&home.join("cache.json"));
        update(&mut cache, &home.join("projects"), None, UNIX_EPOCH);
        // 同じ message.id の行が後から追記されても二重計上しない
        let mut content = fs::read_to_string(&file).unwrap();
        content.push_str(&format!("{line}\n"));
        fs::write(&file, content).unwrap();
        update(&mut cache, &home.join("projects"), None, UNIX_EPOCH);
        let sums = month_model_sums(&cache, &current_month());
        assert_eq!(sums.get("claude-opus-4-8").unwrap().output, 100);
    }

    #[test]
    fn transcript_path_outside_cutoff_is_still_scanned() {
        let home = tmp("transcript");
        let proj = home.join("projects/p1");
        fs::create_dir_all(&proj).unwrap();
        let file = proj.join("sess.jsonl");
        fs::write(&file, format!("{}\n", assistant_line("m1", "claude-haiku-4-5", 7))).unwrap();

        // cutoff を未来にして mtime フィルタから外す
        let future = SystemTime::now() + Duration::from_secs(3600);
        let mut cache = load(&home.join("cache.json"));
        update(&mut cache, &home.join("projects"), Some(&file), future);
        let sums = file_model_sums(&cache, &file.to_string_lossy());
        assert_eq!(sums.get("claude-haiku-4-5").unwrap().output, 7);
    }
}
```

`claude/statusline/src/lib.rs`:
```rust
pub mod cache;
pub mod git;
pub mod input;
pub mod pricing;
pub mod transcript;
```

- [ ] **Step 2: Verify**

```sh
cd claude/statusline && cargo fmt && cargo test --quiet && cargo clippy --all-targets -- -D warnings && cargo fmt -- --check
```

Expected: 累計 21 tests pass; clippy/fmt clean。

- [ ] **Step 3: Commit**

```sh
git add claude/statusline
git commit -m "$(cat <<'EOF'
Add persistent incremental cache for transcript aggregation

ファイル別に読込オフセット・重複排除ハッシュ・月別モデル別集計を
JSON で永続化し、mtime が当月 cutoff 以降のファイルと現セッションの
transcript だけを差分パースする。初回のみフルスキャン、以降は数十 ms
で収まる。保存は PID 付き temp+rename で原子的に行い、複数セッション
並行起動時の tmp 衝突を避ける。
EOF
)"
```

---

### Task 6: 2 行レンダリング (`render.rs`)

**Why:** 確定レイアウトどおりに ANSI カラー付き 2 行を組み立てる。データ欠落時のフォールバック(ctx `–` 表示、rate_limits セグメント省略、単価不明 `?`)もここで実装する。

**Behavior change:** yes(表示の新規実装)
**Discipline:** TDD — ANSI を除去した文字列で厳密比較するテストを先に書く。

**Files:**
- Create: `claude/statusline/src/render.rs`
- Modify: `claude/statusline/src/lib.rs`(`pub mod render;` 追加)

### Steps

- [ ] **Step 1: 失敗するテストを含む `render.rs` を書く(red → green)**

`claude/statusline/src/render.rs`:
```rust
use std::collections::HashMap;
use std::path::Path;

use crate::input::{RateLimitWindow, StatusInput};
use crate::pricing::{cost_usd, PricingTable};
use crate::transcript::TokenSums;

const RESET: &str = "\x1b[0m";
const DIM: &str = "\x1b[2m";
const BOLD: &str = "\x1b[1m";
// dark-daltonized テーマ向け: 赤/緑を避け、青/オレンジで閾値を表す
const BLUE: &str = "\x1b[38;5;75m";
const ORANGE: &str = "\x1b[38;5;208m";

pub struct RenderData<'a> {
    pub input: &'a StatusInput,
    pub branch: Option<String>,
    /// 現セッションのモデル別トークン集計
    pub session_models: HashMap<String, TokenSums>,
    /// 当月のモデル別トークン集計(全プロジェクト)
    pub month_models: HashMap<String, TokenSums>,
    pub pricing: &'a PricingTable,
}

pub fn render(d: &RenderData) -> String {
    format!("{}\n{}", line1(d), line2(d))
}

fn line1(d: &RenderData) -> String {
    let mut parts: Vec<String> = Vec::new();
    if let Some(dir) = d.input.workspace.current_dir.as_deref() {
        let name = Path::new(dir)
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| dir.to_string());
        parts.push(format!("{BOLD}{name}{RESET}"));
    }
    if let Some(b) = &d.branch {
        parts.push(format!("{DIM}\u{2387} {b}{RESET}"));
    }
    if let Some(m) = &d.input.model.display_name {
        parts.push(m.clone());
    }
    parts.push(ctx_segment(d));
    parts.join("  ")
}

fn ctx_segment(d: &RenderData) -> String {
    let Some(free) = d
        .input
        .context_window
        .as_ref()
        .and_then(|c| c.remaining_percentage)
    else {
        return format!("ctx {DIM}\u{2013}{RESET}");
    };
    let color = if free < 10.0 {
        format!("{ORANGE}{BOLD}")
    } else if free < 30.0 {
        ORANGE.to_string()
    } else {
        BLUE.to_string()
    };
    let filled = ((free / 10.0).round() as usize).min(10);
    let bar = "\u{2588}".repeat(filled) + &"\u{2591}".repeat(10 - filled);
    format!("ctx {color}{bar}{RESET} {free:.0}% free")
}

fn line2(d: &RenderData) -> String {
    let mut parts: Vec<String> = Vec::new();
    if let Some(rl) = &d.input.rate_limits {
        if let Some(seg) = rl.five_hour.as_ref().and_then(|w| limit_segment("5h", w, "%H:%M")) {
            parts.push(seg);
        }
        if let Some(seg) = rl
            .seven_day
            .as_ref()
            .and_then(|w| limit_segment("wk", w, "%a %H:%M"))
        {
            parts.push(seg);
        }
    }
    parts.push(session_segment(d));
    parts.push(month_segment(d));
    parts.join("  ")
}

fn limit_segment(label: &str, w: &RateLimitWindow, fmt: &str) -> Option<String> {
    let pct = w.used_percentage?;
    let color = if pct >= 90.0 {
        format!("{ORANGE}{BOLD}")
    } else if pct >= 70.0 {
        ORANGE.to_string()
    } else {
        String::new()
    };
    let reset = w
        .resets_at
        .and_then(|t| chrono::DateTime::from_timestamp(t as i64, 0))
        .map(|t| t.with_timezone(&chrono::Local).format(fmt).to_string())
        .map(|s| format!(" {DIM}\u{21bb}{s}{RESET}"))
        .unwrap_or_default();
    Some(format!("{label} {color}{pct:.0}%{RESET}{reset}"))
}

fn session_segment(d: &RenderData) -> String {
    let mut items: Vec<(String, Option<f64>)> = d
        .session_models
        .iter()
        .map(|(model, sums)| {
            let cost = d.pricing.lookup(model).map(|p| cost_usd(sums, &p));
            (short_name(model), cost)
        })
        .collect();
    items.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    let total = d
        .input
        .cost
        .as_ref()
        .and_then(|c| c.total_cost_usd)
        .unwrap_or_else(|| items.iter().filter_map(|(_, c)| *c).sum());
    let breakdown = items
        .iter()
        .map(|(name, cost)| match cost {
            Some(c) => format!("{name} {}", fmt_usd(*c)),
            None => format!("{name} ?"),
        })
        .collect::<Vec<_>>()
        .join(" | ");
    if breakdown.is_empty() {
        format!("S {}", fmt_usd(total))
    } else {
        format!("S {} {DIM}({breakdown}){RESET}", fmt_usd(total))
    }
}

fn month_segment(d: &RenderData) -> String {
    let mut total = 0.0;
    let mut unpriced = false;
    for (model, sums) in &d.month_models {
        match d.pricing.lookup(model) {
            Some(p) => total += cost_usd(sums, &p),
            None => unpriced = true,
        }
    }
    let mark = if unpriced { "+?" } else { "" };
    format!("M {}{mark}", fmt_usd(total))
}

fn fmt_usd(v: f64) -> String {
    if v >= 100.0 {
        format!("${v:.0}")
    } else if v >= 10.0 {
        format!("${v:.1}")
    } else {
        format!("${v:.2}")
    }
}

/// "claude-" prefix と日付サフィックス(-20251001 等)を落とした短縮名。
fn short_name(model: &str) -> String {
    let s = model.strip_prefix("claude-").unwrap_or(model);
    if let Some(idx) = s.rfind('-') {
        let tail = &s[idx + 1..];
        if tail.len() == 8 && tail.chars().all(|c| c.is_ascii_digit()) {
            return s[..idx].to_string();
        }
    }
    s.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::StatusInput;

    /// ANSI エスケープを除去して見た目のテキストだけにする。
    fn strip_ansi(s: &str) -> String {
        let mut out = String::new();
        let mut in_esc = false;
        for c in s.chars() {
            if in_esc {
                if c == 'm' {
                    in_esc = false;
                }
            } else if c == '\u{1b}' {
                in_esc = true;
            } else {
                out.push(c);
            }
        }
        out
    }

    fn sums(output: u64) -> TokenSums {
        TokenSums {
            output,
            ..Default::default()
        }
    }

    fn full_input() -> StatusInput {
        StatusInput::parse(
            r#"{
            "transcript_path": "/tmp/t.jsonl",
            "model": {"display_name": "Fable 5"},
            "workspace": {"current_dir": "/Users/x/dotfiles"},
            "cost": {"total_cost_usd": 1.23},
            "context_window": {"remaining_percentage": 72.0},
            "rate_limits": {
                "five_hour": {"used_percentage": 41.0, "resets_at": 1751700600},
                "seven_day": {"used_percentage": 23.0, "resets_at": 1751900400}
            }
        }"#,
        )
        .unwrap()
    }

    #[test]
    fn renders_two_lines_with_all_data() {
        let pricing = PricingTable::embedded();
        let mut session = HashMap::new();
        // opus-4-8: 1M output = $25.00 / haiku-4-5: 1M output = $5.00
        session.insert("claude-opus-4-8".to_string(), sums(1_000_000));
        session.insert("claude-haiku-4-5".to_string(), sums(1_000_000));
        let mut month = HashMap::new();
        month.insert("claude-opus-4-8".to_string(), sums(2_000_000));
        let d = RenderData {
            input: &full_input(),
            branch: Some("main".to_string()),
            session_models: session,
            month_models: month,
            pricing: &pricing,
        };
        let out = strip_ansi(&render(&d));
        let lines: Vec<&str> = out.lines().collect();
        assert_eq!(lines.len(), 2);
        assert_eq!(
            lines[0],
            "dotfiles  \u{2387} main  Fable 5  ctx \u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2591}\u{2591}\u{2591} 72% free"
        );
        assert!(lines[1].starts_with("5h 41% \u{21bb}"));
        assert!(lines[1].contains("wk 23% \u{21bb}"));
        assert!(lines[1].contains("S $1.23 (opus-4-8 $25.0 | haiku-4-5 $5.00)"));
        assert!(lines[1].contains("M $50.0"));
    }

    #[test]
    fn degrades_gracefully_without_optional_data() {
        let pricing = PricingTable::embedded();
        let input = StatusInput::parse(r#"{"session_id":"x"}"#).unwrap();
        let d = RenderData {
            input: &input,
            branch: None,
            session_models: HashMap::new(),
            month_models: HashMap::new(),
            pricing: &pricing,
        };
        let out = strip_ansi(&render(&d));
        let lines: Vec<&str> = out.lines().collect();
        assert_eq!(lines[0], "ctx \u{2013}");
        assert_eq!(lines[1], "S $0.00  M $0.00");
    }

    #[test]
    fn unknown_model_shows_question_mark() {
        let pricing = PricingTable::embedded();
        let mut session = HashMap::new();
        session.insert("mystery-model".to_string(), sums(100));
        let mut month = HashMap::new();
        month.insert("mystery-model".to_string(), sums(100));
        let input = StatusInput::parse(r#"{"cost":{"total_cost_usd":0.5}}"#).unwrap();
        let d = RenderData {
            input: &input,
            branch: None,
            session_models: session,
            month_models: month,
            pricing: &pricing,
        };
        let out = strip_ansi(&render(&d));
        assert!(out.contains("mystery-model ?"));
        assert!(out.contains("M $0.00+?"));
    }

    #[test]
    fn short_name_strips_prefix_and_date() {
        assert_eq!(short_name("claude-fable-5"), "fable-5");
        assert_eq!(short_name("claude-haiku-4-5-20251001"), "haiku-4-5");
        assert_eq!(short_name("mystery"), "mystery");
    }

    #[test]
    fn fmt_usd_scales_precision() {
        assert_eq!(fmt_usd(0.9), "$0.90");
        assert_eq!(fmt_usd(45.6), "$45.6");
        assert_eq!(fmt_usd(123.4), "$123");
    }
}
```

`claude/statusline/src/lib.rs`:
```rust
pub mod cache;
pub mod git;
pub mod input;
pub mod pricing;
pub mod render;
pub mod transcript;
```

**注:** `renders_two_lines_with_all_data` の line1 期待値は `parts.join("  ")`(2 スペース)で結合した文字列。テストが FAIL したら期待値ではなく実装のスペースを確認すること。

- [ ] **Step 2: Verify**

```sh
cd claude/statusline && cargo fmt && cargo test --quiet && cargo clippy --all-targets -- -D warnings && cargo fmt -- --check
```

Expected: 累計 26 tests pass; clippy/fmt clean。

- [ ] **Step 3: Commit**

```sh
git add claude/statusline
git commit -m "$(cat <<'EOF'
Add two-line ANSI renderer with graceful degradation

1 行目: ディレクトリ / ブランチ / モデル / context 残量バー。
2 行目: 5h・週次使用量 (リセット時刻付き) / セッションコスト
(モデル別内訳) / 月次トータル。閾値色は dark-daltonized 向けに
青/オレンジ系。欠落データはセグメント省略または – / ? で縮退。
EOF
)"
```

---

### Task 7: オーケストレーション (`lib.rs::run` + `main.rs`) と E2E テスト

**Why:** 各モジュールを結線し、stdin → 2 行出力の全経路を通す。実バイナリを子プロセスとして起動し、fixture の HOME/transcript で出力を検証する E2E テストで「本物の呼び出し形態」を担保する。

**Behavior change:** yes(エントリポイント完成)
**Discipline:** TDD — E2E テストを先に書き、`run` 実装で green にする。

**Files:**
- Modify: `claude/statusline/src/lib.rs`(`run` 関数追加)
- Modify: `claude/statusline/src/main.rs`(`run` を呼ぶ最終形)
- Create: `claude/statusline/tests/integration.rs`

### Steps

- [ ] **Step 1: E2E テストを書く(red)**

`claude/statusline/tests/integration.rs`:
```rust
use std::io::Write;
use std::process::{Command, Stdio};

fn assistant_line(msg_id: &str, model: &str, input: u64, output: u64) -> String {
    let ts = chrono::Local::now().to_rfc3339();
    format!(
        r#"{{"type":"assistant","timestamp":"{ts}","requestId":"req-{msg_id}","message":{{"id":"{msg_id}","model":"{model}","usage":{{"input_tokens":{input},"output_tokens":{output},"cache_read_input_tokens":0,"cache_creation_input_tokens":0}}}}}}"#
    )
}

fn run_binary(home: &std::path::Path, stdin_json: &str) -> String {
    let mut child = Command::new(env!("CARGO_BIN_EXE_claude-statusline"))
        .env("HOME", home)
        .env("XDG_CACHE_HOME", home.join("xdg-cache"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("spawn binary");
    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(stdin_json.as_bytes())
        .unwrap();
    let out = child.wait_with_output().expect("wait binary");
    assert!(out.status.success());
    String::from_utf8(out.stdout).unwrap()
}

fn strip_ansi(s: &str) -> String {
    let mut out = String::new();
    let mut in_esc = false;
    for c in s.chars() {
        if in_esc {
            if c == 'm' {
                in_esc = false;
            }
        } else if c == '\u{1b}' {
            in_esc = true;
        } else {
            out.push(c);
        }
    }
    out
}

#[test]
fn end_to_end_renders_costs_from_transcript() {
    let home = std::env::temp_dir().join(format!("cs-e2e-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&home);
    let proj = home.join(".claude/projects/test-proj");
    std::fs::create_dir_all(&proj).unwrap();

    let transcript = proj.join("session-1.jsonl");
    let opus = assistant_line("m-opus", "claude-opus-4-8", 0, 1_000_000); // $25.00
    let haiku = assistant_line("m-haiku", "claude-haiku-4-5", 1_000_000, 0); // $1.00
    // opus 行を重複させる → dedup が効けば M は $26.0 のまま
    std::fs::write(&transcript, format!("{opus}\n{opus}\n{haiku}\n")).unwrap();

    let stdin_json = format!(
        r#"{{
        "transcript_path": "{}",
        "model": {{"display_name": "Fable 5"}},
        "workspace": {{"current_dir": "{}"}},
        "cost": {{"total_cost_usd": 1.23}},
        "context_window": {{"remaining_percentage": 72.0}},
        "rate_limits": {{
            "five_hour": {{"used_percentage": 41.0, "resets_at": 1751700600}},
            "seven_day": {{"used_percentage": 23.0, "resets_at": 1751900400}}
        }}
    }}"#,
        transcript.display(),
        home.display()
    );

    let out1 = strip_ansi(&run_binary(&home, &stdin_json));
    let lines: Vec<&str> = out1.lines().collect();
    assert_eq!(lines.len(), 2, "expected 2 lines, got: {out1}");
    assert!(lines[0].contains("Fable 5"));
    assert!(lines[0].contains("72% free"));
    assert!(lines[1].contains("5h 41%"));
    assert!(lines[1].contains("wk 23%"));
    assert!(lines[1].contains("S $1.23"));
    assert!(lines[1].contains("opus-4-8 $25.0"));
    assert!(lines[1].contains("haiku-4-5 $1.00"));
    assert!(lines[1].contains("M $26.0"), "dedup failed?: {out1}");

    // 2 回目 (キャッシュ経由・差分なし) も同一出力
    let out2 = strip_ansi(&run_binary(&home, &stdin_json));
    assert_eq!(out1, out2);

    // キャッシュファイルが生成されている
    assert!(home
        .join("xdg-cache/claude-statusline/cache.json")
        .exists());
}

#[test]
fn survives_empty_stdin() {
    let home = std::env::temp_dir().join(format!("cs-e2e-empty-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&home);
    std::fs::create_dir_all(&home).unwrap();
    let out = run_binary(&home, "");
    assert!(!out.trim().is_empty());
}
```

Run: `cd claude/statusline && cargo test --quiet --test integration`
Expected: FAIL(`run` 未実装のため出力が `(wip)`)。

- [ ] **Step 2: `lib.rs` に `run` を実装する(green)**

`claude/statusline/src/lib.rs` 全体:
```rust
pub mod cache;
pub mod git;
pub mod input;
pub mod pricing;
pub mod render;
pub mod transcript;

use std::path::{Path, PathBuf};

/// stdin JSON を受け取り、statusline の 2 行を返す。
/// 個々のデータソースの失敗は縮退表示に落とし、panic しない。
pub fn run(stdin_json: &str) -> String {
    let input = input::StatusInput::parse(stdin_json).unwrap_or_default();

    let home = std::env::var("HOME").unwrap_or_default();
    let home = Path::new(&home);

    let mut pricing = pricing::PricingTable::embedded();
    pricing.load_overrides(&home.join(".claude/statusline.toml"));

    let cache_dir = std::env::var("XDG_CACHE_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| home.join(".cache"));
    let cache_path = cache_dir.join("claude-statusline/cache.json");

    let now = chrono::Local::now();
    let current_month = now.format("%Y-%m").to_string();
    let cutoff = month_cutoff(&now);

    let mut cache = cache::load(&cache_path);
    let transcript = input.transcript_path.as_deref().map(Path::new);
    cache::update(&mut cache, &home.join(".claude/projects"), transcript, cutoff);
    cache::save(&cache_path, &cache);

    let session_models = input
        .transcript_path
        .as_deref()
        .map(|p| cache::file_model_sums(&cache, p))
        .unwrap_or_default();
    let month_models = cache::month_model_sums(&cache, &current_month);

    let branch = input
        .workspace
        .current_dir
        .as_deref()
        .and_then(|d| git::current_branch(Path::new(d)));

    render::render(&render::RenderData {
        input: &input,
        branch,
        session_models,
        month_models,
        pricing: &pricing,
    })
}

/// 当月 1 日 0:00 (ローカル) − 26h。mtime フィルタ用の安全マージン付き cutoff。
/// (mtime の TZ ずれ・月境界を跨ぐセッションを取りこぼさないための余裕)
fn month_cutoff(now: &chrono::DateTime<chrono::Local>) -> std::time::SystemTime {
    use chrono::{Datelike, TimeZone};
    let start = chrono::Local
        .with_ymd_and_hms(now.year(), now.month(), 1, 0, 0, 0)
        .single()
        .map(|d| d.timestamp())
        .unwrap_or(0);
    std::time::UNIX_EPOCH
        + std::time::Duration::from_secs((start.max(0) as u64).saturating_sub(26 * 3600))
}
```

`claude/statusline/src/main.rs` 全体:
```rust
fn main() {
    use std::io::Read;
    let mut buf = String::new();
    let _ = std::io::stdin().read_to_string(&mut buf);
    println!("{}", claude_statusline::run(&buf));
}
```

- [ ] **Step 3: Verify**

```sh
cd claude/statusline && cargo fmt && cargo test --quiet && cargo clippy --all-targets -- -D warnings && cargo fmt -- --check
```

Expected: 累計 28 tests pass(unit 26 + integration 2); clippy/fmt clean。

- [ ] **Step 4: release ビルドと性能の目視確認**

```sh
cd claude/statusline && cargo build --release --quiet && \
  echo '{"transcript_path":null}' | time ./target/release/claude-statusline
```

Expected: 2 行(縮退表示)が出力され、real が 0.1 秒未満。

- [ ] **Step 5: Commit**

```sh
git add claude/statusline
git commit -m "$(cat <<'EOF'
Wire up statusline pipeline with end-to-end tests

stdin JSON → 単価表ロード → キャッシュ増分更新 → セッション/月次
集計 → 2 行レンダリングを結線。実バイナリを fixture HOME で起動する
E2E テストで dedup 込みのコスト計算と 2 回目実行の安定性を検証する。
EOF
)"
```

---

### Task 8: install.sh 統合と statusline.toml テンプレート

**Why:** 「dotfiles 編集 → install.sh で反映」の単一フローにバイナリを乗せる。cargo が無い環境では警告してスキップし、`statusLine` 未設定を検知したら貼り付け用スニペットを案内する(settings.json 自体は触らない — デバイス固有ポリシー維持)。

**Behavior change:** yes(install.sh の機能追加)
**Discipline:** シェルスクリプトのため TDD 対象外。`bash -n` の構文チェックと実行による動作確認で検証する。

**Files:**
- Create: `claude/statusline.toml`
- Modify: `claude/install.sh`

### Steps

- [ ] **Step 1: `claude/statusline.toml` テンプレートを作成する**

```toml
# claude-statusline の単価上書き設定。
# バイナリ埋め込みの価格表を [pricing."<model-id>"] で追加・上書きできる。
# 値は $/MTok。5 フィールドすべて必須(欠けるとそのエントリは無視される)。
# モデル ID は最長 prefix 一致で解決される。
#
# 例: Sonnet 5 の introductory 価格 (〜2026-08-31)
# [pricing."claude-sonnet-5"]
# input = 2.0
# output = 10.0
# cache_write_5m = 2.5
# cache_write_1h = 4.0
# cache_read = 0.2
```

- [ ] **Step 2: `claude/install.sh` を以下の内容に更新する**

```bash
#!/bin/bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
TARGET_DIR="$HOME/.claude"

echo "Installing Claude Code configuration..."
echo "  Source: $SCRIPT_DIR"
echo "  Target: $TARGET_DIR"

mkdir -p "$TARGET_DIR"

# CLAUDE.md
cp "$SCRIPT_DIR/CLAUDE.md" "$TARGET_DIR/CLAUDE.md"
echo "  Copied CLAUDE.md"

# skills/
rsync -a --delete "$SCRIPT_DIR/skills/" "$TARGET_DIR/skills/"
echo "  Synced skills/"

# agents/
rsync -a --delete "$SCRIPT_DIR/agents/" "$TARGET_DIR/agents/"
echo "  Synced agents/"

# statusline.toml (単価上書き設定)
cp "$SCRIPT_DIR/statusline.toml" "$TARGET_DIR/statusline.toml"
echo "  Copied statusline.toml"

# claude-statusline (Rust バイナリ)
if command -v cargo >/dev/null 2>&1; then
    echo "  Building claude-statusline (release)..."
    (cd "$SCRIPT_DIR/statusline" && cargo build --release --quiet)
    mkdir -p "$HOME/.local/bin"
    cp "$SCRIPT_DIR/statusline/target/release/claude-statusline" "$HOME/.local/bin/claude-statusline"
    echo "  Installed claude-statusline to ~/.local/bin"
else
    echo "  WARNING: cargo not found; skipped building claude-statusline"
fi

# statusLine 未設定なら貼り付け用スニペットを案内する (settings.json は編集しない)
if ! grep -q '"statusLine"' "$TARGET_DIR/settings.json" 2>/dev/null; then
    cat <<EOF

  statusLine is not configured yet.
  Add this to $TARGET_DIR/settings.json:

    "statusLine": {
      "type": "command",
      "command": "$HOME/.local/bin/claude-statusline"
    }
EOF
fi

echo ""
echo "Done. The following were NOT copied (device-specific):"
echo "  settings.json, projects/, sessions/, history.jsonl, cache/, plugins/, todos/"
```

- [ ] **Step 3: 構文チェックと実行確認**

```sh
bash -n claude/install.sh && bash claude/install.sh
```

Expected:
- `Building claude-statusline (release)...` → `Installed claude-statusline to ~/.local/bin` が出力される
- `~/.local/bin/claude-statusline` が実行可能ファイルとして存在する
- `~/.claude/statusline.toml` がコピーされている
- 現デバイスの settings.json には `statusLine` が無いため、スニペット案内が表示される

確認コマンド:
```sh
test -x "$HOME/.local/bin/claude-statusline" && echo BIN_OK
test -f "$HOME/.claude/statusline.toml" && echo TOML_OK
echo '{}' | "$HOME/.local/bin/claude-statusline"
```
Expected: `BIN_OK` / `TOML_OK`、最後のコマンドは縮退表示の 2 行(`ctx –` を含む)を出力する。

- [ ] **Step 4: Commit**

```sh
git add claude/install.sh claude/statusline.toml
git commit -m "$(cat <<'EOF'
Install claude-statusline via install.sh

cargo があれば release ビルドして ~/.local/bin に配置、無ければ警告
してスキップする。単価上書き用 statusline.toml も ~/.claude/ に
コピーする。settings.json は編集せず、statusLine 未設定を検知した
場合のみ貼り付け用スニペットを表示する (デバイス固有ポリシー維持)。
EOF
)"
```

---

## Final verification (after all tasks)

```sh
cd claude/statusline && cargo fmt -- --check && cargo clippy --all-targets -- -D warnings && cargo test --quiet && cargo build --release --quiet
bash -n claude/install.sh
# スモークテスト: 実際の transcript を食わせる (HOME はそのまま)
echo "{\"transcript_path\":\"$(ls -t ~/.claude/projects/*/*.jsonl | head -1)\",\"model\":{\"display_name\":\"Test\"},\"workspace\":{\"current_dir\":\"$PWD\"},\"context_window\":{\"remaining_percentage\":50.0}}" | ./target/release/claude-statusline
```

Expected: 28 tests all pass; clippy/fmt clean; release build 成功; スモークテストが 2 行を出力し、2 行目に `S $` / `M $` と実際の金額が表示される(初回はフルスキャンで数秒かかる場合がある。2 回目は即時)。

## Post-/review iteration

Reserved for fix tasks appended by Claude Code after `/review` produces actionable items. Empty until `/review` runs.

(See CLAUDE.md "Core Flow" for the autonomous review feedback loop.)

## Push and PR

```sh
git push -u origin feature/claude-statusline
gh pr create --base main --title "Add Rust statusline with usage and cost display" --body "..."
```

PR description には各コミットの役割、キャッシュ設計(増分オフセット + 重複排除)、単価表の根拠(2026-07 時点の公表価格、TOML で上書き可)を記載する。

## Out of scope

- 月次コストのモデル別内訳表示(キャッシュはモデル別保持済みのため、将来は render の変更のみで対応可能)
- settings.json の自動マージ(design discussion で却下 — デバイス固有ポリシー維持)
- LiteLLM 等からの単価自動取得(ネットワーク依存を避ける)
- 全期間累計コスト表示
- Sonnet 5 introductory 価格の期限付き自動切替(必要なら statusline.toml で手動設定)
- `refreshInterval` による時刻ベースの再描画設定
- Linux など他デバイスでの動作検証(このデバイスは macOS のみ)
- 1M コンテキストの 2 段単価(Opus 4.7+/Fable 5 は premium なしと公式ドキュメントに明記。将来必要になったモデルは TOML で単価ごと上書き)

## Alternative Solutions Considered

- **ccusage を statusline から呼び出す**: 既存ツールで月次集計可能。**Rejected because**: Node/Bun 起動オーバーヘッドが 300ms 予算を超過し、外部ランタイム依存が増える。Rust 自前実装の増分キャッシュのほうが速く自己完結。
- **settings.json を install.sh が jq で自動マージ**: 新デバイス完全ワンコマンド化。**Rejected because**: 「settings.json はデバイス固有・install.sh は触らない」という既存ポリシーを破り、マージ事故の故障半径が大きい。スニペット案内で利便性の大半を確保。
- **cargo install --path による独立管理**: install.sh を汚さない。**Rejected because**: 「dotfiles 編集 → install.sh」の単一フローが崩れ、バイナリだけ古いままの「静かな陳腐化」が起きる(コスト表示ツールとして最悪の壊れ方)。
- **単価表を LiteLLM の価格 JSON から取得**: 新モデル・改定に自動追従。**Rejected because**: statusline はブロッキング実行のためネットワーク取得の設計(TTL・オフライン fallback)が過剰。埋め込み + TOML 上書きで十分。
- **全期間累計コスト**: ccusage 同等の情報量。**Rejected because**: 初回フルスキャン 113MB+ とキャッシュ肥大化に見合う価値がない。月次スコープが要望(design discussion で確定)。
