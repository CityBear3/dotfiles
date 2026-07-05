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
    assert!(home.join("xdg-cache/claude-statusline/cache.json").exists());
}

#[test]
fn survives_empty_stdin() {
    let home = std::env::temp_dir().join(format!("cs-e2e-empty-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&home);
    std::fs::create_dir_all(&home).unwrap();
    let out = strip_ansi(&run_binary(&home, ""));
    let lines: Vec<&str> = out.lines().collect();
    assert_eq!(lines.len(), 2, "expected degraded 2-line output: {out}");
    assert!(lines[0].contains("ctx \u{2013}"));
    assert!(lines[1].contains("S $0.00"));
}
