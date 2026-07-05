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
    cache::update(
        &mut cache,
        &home.join(".claude/projects"),
        transcript,
        cutoff,
    );
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
