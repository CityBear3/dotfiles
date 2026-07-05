use serde::Deserialize;
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

impl PricingTable {
    pub fn embedded() -> Self {
        Self {
            entries: EMBEDDED.iter().map(|(k, v)| (k.to_string(), *v)).collect(),
        }
    }

    /// TOML(全 5 フィールド必須)で追加・上書き。フィールド欠損・型違いの
    /// エントリはそのエントリのみ無視する(statusline.toml 記載の契約)。
    /// ファイル自体が読めない・TOML 構文が壊れている場合は全体を無視する。
    pub fn load_overrides(&mut self, path: &Path) {
        let Ok(text) = std::fs::read_to_string(path) else {
            return;
        };
        let Ok(table) = text.parse::<toml::Table>() else {
            return;
        };
        let Some(pricing) = table.get("pricing").and_then(|v| v.as_table()) else {
            return;
        };
        for (id, entry) in pricing {
            let Ok(price) = entry.clone().try_into::<ModelPrice>() else {
                continue;
            };
            if let Some(e) = self.entries.iter_mut().find(|(k, _)| k == id) {
                e.1 = price;
            } else {
                self.entries.push((id.clone(), price));
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

        let path = std::env::temp_dir().join(format!("cs-broken-{}.toml", std::process::id()));
        std::fs::write(&path, "not [ valid toml").unwrap();
        t.load_overrides(&path);
        assert_eq!(t.lookup("claude-opus-4-8").unwrap().input, 5.0);
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn partial_entry_is_ignored_but_complete_entries_apply() {
        let path = std::env::temp_dir().join(format!("cs-partial-{}.toml", std::process::id()));
        std::fs::write(
            &path,
            r#"
[pricing."claude-opus-4-8"]
input = 1.0
output = 2.0
cache_write_5m = 1.25
cache_write_1h = 2.0
cache_read = 0.1

[pricing."claude-haiku-4-5"]
input = 9.0
output = 9.0
cache_write_5m = 9.0
cache_write_1h = 9.0
"#,
        )
        .unwrap();
        let mut t = PricingTable::embedded();
        t.load_overrides(&path);
        // 完備エントリは適用される
        assert_eq!(t.lookup("claude-opus-4-8").unwrap().input, 1.0);
        // cache_read 欠損エントリはそのエントリのみ無視(埋め込み価格のまま)
        assert_eq!(t.lookup("claude-haiku-4-5").unwrap().input, 1.0);
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn type_mismatched_entry_is_ignored() {
        let path =
            std::env::temp_dir().join(format!("cs-typemismatch-{}.toml", std::process::id()));
        std::fs::write(
            &path,
            r#"
[pricing."claude-sonnet-5"]
input = "abc"
output = 3.0
cache_write_5m = 3.75
cache_write_1h = 6.0
cache_read = 0.3
"#,
        )
        .unwrap();
        let mut t = PricingTable::embedded();
        t.load_overrides(&path);
        // input が型違い(文字列)のエントリはそのエントリのみ無視(埋め込み価格のまま)
        assert_eq!(t.lookup("claude-sonnet-5").unwrap().input, 3.0);
        let _ = std::fs::remove_file(&path);
    }
}
