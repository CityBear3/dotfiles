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
        if let Some(seg) = rl
            .five_hour
            .as_ref()
            .and_then(|w| limit_segment("5h", w, "%H:%M"))
        {
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
    // 空の f64 sum は -0.0 を返し "$-0.00" になるため正のゼロへ正規化する。
    let v = if v == 0.0 { 0.0 } else { v };
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

    #[test]
    fn threshold_colors_are_applied() {
        let pricing = PricingTable::embedded();
        let input = StatusInput::parse(
            r#"{
            "context_window": {"remaining_percentage": 25.0},
            "rate_limits": {"five_hour": {"used_percentage": 95.0, "resets_at": 1751700600}}
        }"#,
        )
        .unwrap();
        let d = RenderData {
            input: &input,
            branch: None,
            session_models: HashMap::new(),
            month_models: HashMap::new(),
            pricing: &pricing,
        };
        let raw = render(&d);
        let line1 = raw.lines().next().unwrap();
        let line2 = raw.lines().nth(1).unwrap();
        // ctx free 25% → 素の ORANGE(BOLD なし)。line1 に限定して 5h への吸収を防ぐ
        assert!(line1.contains(ORANGE));
        assert!(!line1.contains(BOLD));
        // 5h 95% → ORANGE+BOLD
        assert!(line2.contains("\u{1b}[38;5;208m\u{1b}[1m"));
    }

    #[test]
    fn context_window_with_null_percentage_degrades() {
        let pricing = PricingTable::embedded();
        let input =
            StatusInput::parse(r#"{"context_window":{"remaining_percentage":null}}"#).unwrap();
        let d = RenderData {
            input: &input,
            branch: None,
            session_models: HashMap::new(),
            month_models: HashMap::new(),
            pricing: &pricing,
        };
        let out = strip_ansi(&render(&d));
        assert!(out.lines().next().unwrap().contains("ctx \u{2013}"));
    }
}
