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
        let Ok(meta) = fs::metadata(path) else {
            continue;
        };
        let size = meta.len();
        let entry = cache.files.entry(key).or_default();
        if size < entry.offset {
            // truncate / 差し替え → 全再走査
            *entry = FileEntry::default();
        }
        if size == entry.offset {
            continue;
        }
        let Ok(mut f) = fs::File::open(path) else {
            continue;
        };
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
        fs::write(
            &file,
            format!("{}\n", assistant_line("m1", "claude-opus-4-8", 100)),
        )
        .unwrap();

        let mut cache = load(&home.join("cache.json"));
        update(&mut cache, &home.join("projects"), None, UNIX_EPOCH);
        let sums = month_model_sums(&cache, &current_month());
        assert_eq!(sums.get("claude-opus-4-8").unwrap().output, 100);

        // 追記 → 差分のみ反映
        let mut content = fs::read_to_string(&file).unwrap();
        content.push_str(&format!(
            "{}\n",
            assistant_line("m2", "claude-opus-4-8", 50)
        ));
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
        fs::write(
            &file,
            format!("{}\n", assistant_line("m1", "claude-haiku-4-5", 7)),
        )
        .unwrap();

        // cutoff を未来にして mtime フィルタから外す
        let future = SystemTime::now() + Duration::from_secs(3600);
        let mut cache = load(&home.join("cache.json"));
        update(&mut cache, &home.join("projects"), Some(&file), future);
        let sums = file_model_sums(&cache, &file.to_string_lossy());
        assert_eq!(sums.get("claude-haiku-4-5").unwrap().output, 7);
    }
}
