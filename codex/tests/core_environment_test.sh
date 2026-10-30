#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
checker="$script_dir/../bin/check-core-environment"
source_config="$script_dir/../config.toml"
test_root="$(mktemp -d "${TMPDIR:-/tmp}/codex-core-test.XXXXXX")"
trap 'rm -rf "$test_root"' EXIT
trap 'exit 130' HUP INT TERM
mkdir -p "$test_root/bin"
cat > "$test_root/bin/codex" <<'SH'
#!/bin/sh
printf 'codex-cli %s\n' "$CORE_TEST_VERSION"
SH
cat > "$test_root/bin/curl" <<'SH'
#!/bin/sh
printf '%s\n' "$*" >> "$CORE_TEST_CURL_LOG"
if [ "${CORE_TEST_NETWORK_FAILURE:-0}" = 1 ]; then exit 22; fi
cat "$CORE_TEST_UPSTREAM"
SH
chmod +x "$test_root/bin/codex" "$test_root/bin/curl"

baseline_names='"PATH","SHELL","TMPDIR","TEMP","TMP","HOME","LANG","LC_ALL","LC_CTYPE","LOGNAME","USER",'
failures=0

run_case() {
    local name=$1 version=$2 names=$3 expected=$4 requests=$5 network_failure=${6:-0}
    local case_root="$test_root/$name" status=0
    mkdir -p "$case_root"
    printf 'const UNIX_CORE_ENV_VARS: &[&str] = &[\n%s\n];\n' "$names" > "$case_root/upstream"
    : > "$case_root/curl.log"
    # Act: both commands are controlled fixtures; never contact the network or live Codex.
    PATH="$test_root/bin:/usr/bin:/bin" \
        CORE_TEST_VERSION="$version" \
        CORE_TEST_UPSTREAM="$case_root/upstream" \
        CORE_TEST_CURL_LOG="$case_root/curl.log" \
        CORE_TEST_NETWORK_FAILURE="$network_failure" \
        bash "$checker" "$source_config" > "$case_root/output" 2>&1 || status=$?
    # Assert
    if [[ "$status" -ne 0 ]] || ! grep -Fq "$expected" "$case_root/output" \
        || [[ "$(wc -l < "$case_root/curl.log" | tr -d ' ')" != "$requests" ]] \
        || { [[ "$requests" != 0 ]] && ! grep -Fq \
            "https://raw.githubusercontent.com/openai/codex/rust-v$version/codex-rs/protocol/src/shell_environment.rs" \
            "$case_root/curl.log"; }; then
        printf 'FAIL: %s (status=%s)\n' "$name" "$status" >&2
        cat "$case_root/output" >&2
        failures=$((failures + 1))
    else
        printf 'PASS: %s\n' "$name"
    fi
}

run_case reviewed-version 0.165.0 "$baseline_names" 'CORE OK' 0
run_case newer-same-core 0.166.0 "$baseline_names" 'CORE OK' 1
run_case added-core-variable 0.166.0 "$baseline_names\"NEW_CORE\"," 'added: NEW_CORE' 1
run_case removed-core-variable 0.166.0 '"PATH",' 'removed: HOME' 1
run_case offline 0.166.0 "$baseline_names" 'CORE UNVERIFIED' 1 1
run_case unsupported-source 0.166.0 'include_something_else!()' 'CORE UNVERIFIED' 1
run_case missing-version unavailable "$baseline_names" 'CORE UNVERIFIED' 0

# Exercise the actual curl dependency with a local-only transport fixture. A
# curlrc remote-name setting must not overwrite an unrelated cwd file.
case_root="$test_root/curlrc"
mkdir -p "$case_root/bin" "$case_root/curl-home" "$case_root/remote" "$case_root/work"
ln -s "$test_root/bin/codex" "$case_root/bin/codex"
cat > "$case_root/bin/curl" <<'SH'
#!/usr/bin/env bash
arguments=()
for argument in "$@"; do
    case "$argument" in
        https://raw.githubusercontent.com/openai/codex/*)
            arguments+=("file://$CORE_TEST_UPSTREAM") ;;
        *) arguments+=("$argument") ;;
    esac
done
exec /usr/bin/curl "${arguments[@]}"
SH
chmod +x "$case_root/bin/curl"
printf '%s\n' 'remote-name' > "$case_root/curl-home/.curlrc"
printf 'const UNIX_CORE_ENV_VARS: &[&str] = &[%s];\n' "$baseline_names" \
    > "$case_root/remote/shell_environment.rs"
printf '%s\n' 'unrelated user source' > "$case_root/expected"
cp "$case_root/expected" "$case_root/work/shell_environment.rs"
status=0
(
    cd "$case_root/work"
    PATH="$case_root/bin:/usr/bin:/bin" \
        CURL_HOME="$case_root/curl-home" \
        CORE_TEST_VERSION=0.166.0 \
        CORE_TEST_UPSTREAM="$case_root/remote/shell_environment.rs" \
        bash "$checker" "$source_config"
) > "$case_root/output" 2>&1 || status=$?
if [[ "$status" -ne 0 ]] || ! grep -Fq 'CORE OK' "$case_root/output" \
    || ! cmp -s "$case_root/expected" "$case_root/work/shell_environment.rs"; then
    printf '%s\n' 'FAIL: curlrc must not write the response into the working directory' >&2
    cat "$case_root/output" >&2
    failures=$((failures + 1))
else
    printf '%s\n' 'PASS: curlrc cannot overwrite unrelated files'
fi

if [[ "$failures" -ne 0 ]]; then
    exit 1
fi
printf '%s\n' 'PASS: Codex core environment checks'
