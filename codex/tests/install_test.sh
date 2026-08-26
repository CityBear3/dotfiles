#!/usr/bin/env bash
set -euo pipefail

run_fake_cargo() {
    case "${CODEX_INSTALL_TEST_EXPECTED_LINK_STATE:-ignore}" in
        absent)
            if [[ -e "$CODEX_INSTALL_TEST_DESTINATION" || -L "$CODEX_INSTALL_TEST_DESTINATION" ]]; then
                printf 'helper link existed before controlled Cargo invocation\n' >&2
                exit 97
            fi
            ;;
        exact)
            if [[ ! -L "$CODEX_INSTALL_TEST_DESTINATION" ]] \
                || [[ "$(readlink "$CODEX_INSTALL_TEST_DESTINATION")" != "$CODEX_INSTALL_TEST_EXPECTED_SOURCE" ]]; then
                printf 'exact helper link was not preserved before controlled Cargo invocation\n' >&2
                exit 98
            fi
            ;;
        ignore)
            ;;
        *)
            printf 'unknown expected helper-link state: %s\n' \
                "$CODEX_INSTALL_TEST_EXPECTED_LINK_STATE" >&2
            exit 99
            ;;
    esac

    {
        printf '%s\n' 'CALL'
        for argument in "$@"; do
            printf 'ARG=%s\n' "$argument"
        done
    } >> "$CODEX_INSTALL_TEST_CARGO_LOG"
    exit "${CODEX_INSTALL_TEST_CARGO_STATUS:-0}"
}

if [[ "${CODEX_INSTALL_TEST_FAKE_CARGO:-0}" == 1 ]]; then
    run_fake_cargo "$@"
fi

fail() {
    printf 'FAIL: %s\n' "$1" >&2
    exit 1
}

assert_status() {
    local expected_status=$1
    local actual_status=$2
    local label=$3

    if [[ "$actual_status" -ne "$expected_status" ]]; then
        fail "$label: expected status $expected_status, got $actual_status"
    fi
}

assert_contains() {
    local file=$1
    local expected=$2
    local label=$3

    if ! grep -Fq -- "$expected" "$file"; then
        printf 'FAIL: %s\n' "$label" >&2
        sed -n '1,120p' "$file" >&2
        exit 1
    fi
}

assert_file_equals() {
    local expected_file=$1
    local actual_file=$2
    local label=$3

    if ! cmp -s "$expected_file" "$actual_file"; then
        printf 'FAIL: %s\n' "$label" >&2
        diff -u "$expected_file" "$actual_file" >&2 || :
        exit 1
    fi
}

assert_absent() {
    local path=$1
    local label=$2

    if [[ -e "$path" || -L "$path" ]]; then
        fail "$label: expected path to remain absent: $path"
    fi
}

assert_cargo_call_count() {
    local expected_count=$1
    local call_log=$2
    local label=$3
    local actual_count

    actual_count="$(grep -c '^CALL$' "$call_log" || :)"
    if [[ "$actual_count" -ne "$expected_count" ]]; then
        fail "$label: expected $expected_count controlled Cargo call(s), got $actual_count"
    fi
}

lstat_fingerprint() {
    stat -f '%d:%i:%p:%m' "$1"
}

test_successful_install_creates_link_after_rust() {
    # Arrange
    local case_root="$test_root/success"
    local case_home="$case_root/home"
    local fake_bin="$case_root/bin"
    local cargo_log="$case_root/cargo.log"
    local destination="$case_home/.local/bin/codex-upgrade"
    local expected_file="$case_root/expected"

    mkdir -p "$case_home" "$fake_bin"
    ln -s "$test_script" "$fake_bin/cargo"

    # Act
    set +e
    (
        cd "$case_root"
        HOME="$case_home" \
            PATH="$fake_bin:/usr/bin:/bin" \
            CODEX_INSTALL_TEST_FAKE_CARGO=1 \
            CODEX_INSTALL_TEST_CARGO_LOG="$cargo_log" \
            CODEX_INSTALL_TEST_DESTINATION="$destination" \
            CODEX_INSTALL_TEST_EXPECTED_LINK_STATE=absent \
            "$launcher" install \
                --codex-home "$case_root/codex home" \
                --state-dir "$case_root/state"
    ) > "$case_root/stdout" 2> "$case_root/stderr"
    local status=$?
    set -e

    # Assert
    if [[ "$status" -ne 0 ]]; then
        sed -n '1,120p' "$case_root/stderr" >&2
    fi
    assert_status 0 "$status" 'successful install'
    if [[ ! -L "$destination" ]]; then
        fail 'successful install: expected helper link was not created'
    fi
    if [[ "$(readlink "$destination")" != "$expected_source" ]]; then
        fail 'successful install: helper link target was not the expected absolute source'
    fi
    assert_contains "$case_root/stdout" \
        "CREATE $destination -> $expected_source" \
        'successful install: missing CREATE action'
    printf '%s\n' \
        'CALL' \
        'ARG=run' \
        'ARG=--quiet' \
        'ARG=--locked' \
        'ARG=--release' \
        'ARG=--manifest-path' \
        "ARG=$manifest_path" \
        'ARG=--' \
        'ARG=install' \
        'ARG=--codex-home' \
        "ARG=$case_root/codex home" \
        'ARG=--state-dir' \
        "ARG=$case_root/state" > "$expected_file"
    assert_file_equals "$expected_file" "$cargo_log" \
        'successful install: Cargo argv boundaries changed'
}

test_rust_failure_does_not_create_link() {
    # Arrange
    local case_root="$test_root/rust-failure"
    local case_home="$case_root/home"
    local fake_bin="$case_root/bin"
    local cargo_log="$case_root/cargo.log"
    local destination="$case_home/.local/bin/codex-upgrade"

    mkdir -p "$case_home" "$fake_bin"
    ln -s "$test_script" "$fake_bin/cargo"

    # Act
    set +e
    HOME="$case_home" \
        PATH="$fake_bin:/usr/bin:/bin" \
        CODEX_INSTALL_TEST_FAKE_CARGO=1 \
        CODEX_INSTALL_TEST_CARGO_LOG="$cargo_log" \
        CODEX_INSTALL_TEST_CARGO_STATUS=23 \
        CODEX_INSTALL_TEST_DESTINATION="$destination" \
        CODEX_INSTALL_TEST_EXPECTED_LINK_STATE=absent \
        "$launcher" install \
        > "$case_root/stdout" 2> "$case_root/stderr"
    local status=$?
    set -e

    # Assert
    assert_status 23 "$status" 'Rust failure'
    assert_absent "$case_home/.local" 'Rust failure'
    assert_cargo_call_count 1 "$cargo_log" 'Rust failure'
}

test_existing_exact_link_is_no_op() {
    # Arrange
    local case_root="$test_root/exact-link"
    local case_home="$case_root/home"
    local fake_bin="$case_root/bin"
    local cargo_log="$case_root/cargo.log"
    local destination="$case_home/.local/bin/codex-upgrade"
    local before_fingerprint
    local after_fingerprint

    mkdir -p "$(dirname "$destination")" "$fake_bin"
    ln -s "$expected_source" "$destination"
    ln -s "$test_script" "$fake_bin/cargo"
    before_fingerprint="$(lstat_fingerprint "$destination")"

    # Act
    set +e
    HOME="$case_home" \
        PATH="$fake_bin:/usr/bin:/bin" \
        CODEX_INSTALL_TEST_FAKE_CARGO=1 \
        CODEX_INSTALL_TEST_CARGO_LOG="$cargo_log" \
        CODEX_INSTALL_TEST_DESTINATION="$destination" \
        CODEX_INSTALL_TEST_EXPECTED_LINK_STATE=exact \
        CODEX_INSTALL_TEST_EXPECTED_SOURCE="$expected_source" \
        "$launcher" > "$case_root/stdout" 2> "$case_root/stderr"
    local status=$?
    set -e

    # Assert
    assert_status 0 "$status" 'exact link no-op'
    if [[ ! -L "$destination" ]] || [[ "$(readlink "$destination")" != "$expected_source" ]]; then
        fail 'exact link no-op: expected link changed'
    fi
    after_fingerprint="$(lstat_fingerprint "$destination")"
    if [[ "$after_fingerprint" != "$before_fingerprint" ]]; then
        fail 'exact link no-op: link identity, mode, or mtime changed'
    fi
    assert_cargo_call_count 1 "$cargo_log" 'exact link no-op'
    assert_contains "$case_root/stdout" \
        "NO-OP $destination -> $expected_source" \
        'exact link no-op: missing NO-OP action'
}

test_dry_run_reports_without_mutation() {
    # Arrange
    local case_root="$test_root/dry-run"
    local case_home="$case_root/home"
    local fake_bin="$case_root/bin"
    local cargo_log="$case_root/cargo.log"
    local destination="$case_home/.local/bin/codex-upgrade"
    local expected_file="$case_root/expected"

    mkdir -p "$case_home" "$fake_bin"
    ln -s "$test_script" "$fake_bin/cargo"

    # Act
    set +e
    HOME="$case_home" \
        PATH="$fake_bin:/usr/bin:/bin" \
        CODEX_INSTALL_TEST_FAKE_CARGO=1 \
        CODEX_INSTALL_TEST_CARGO_LOG="$cargo_log" \
        CODEX_INSTALL_TEST_DESTINATION="$destination" \
        CODEX_INSTALL_TEST_EXPECTED_LINK_STATE=absent \
        "$launcher" --dry-run \
        > "$case_root/stdout" 2> "$case_root/stderr"
    local status=$?
    set -e

    # Assert
    assert_status 0 "$status" 'dry-run'
    assert_absent "$case_home/.local" 'dry-run'
    assert_contains "$case_root/stdout" \
        "CREATE $destination -> $expected_source" \
        'dry-run: missing CREATE preview'
    assert_cargo_call_count 1 "$cargo_log" 'dry-run'
    printf '%s\n' \
        'CALL' \
        'ARG=run' \
        'ARG=--quiet' \
        'ARG=--locked' \
        'ARG=--release' \
        'ARG=--manifest-path' \
        "ARG=$manifest_path" \
        'ARG=--' \
        'ARG=--dry-run' > "$expected_file"
    assert_file_equals "$expected_file" "$cargo_log" \
        'dry-run: Cargo argv boundaries changed'
}

test_unexpected_destination_is_preserved_as_conflict() {
    # Arrange
    local case_root="$test_root/conflict"
    local case_home="$case_root/home"
    local fake_bin="$case_root/bin"
    local cargo_log="$case_root/cargo.log"
    local destination="$case_home/.local/bin/codex-upgrade"
    local expected_file="$case_root/expected"
    local before_fingerprint
    local after_fingerprint

    mkdir -p "$(dirname "$destination")" "$fake_bin"
    printf '%s\n' 'user-owned command' > "$destination"
    printf '%s\n' 'user-owned command' > "$expected_file"
    ln -s "$test_script" "$fake_bin/cargo"
    before_fingerprint="$(lstat_fingerprint "$destination")"

    # Act
    set +e
    HOME="$case_home" \
        PATH="$fake_bin:/usr/bin:/bin" \
        CODEX_INSTALL_TEST_FAKE_CARGO=1 \
        CODEX_INSTALL_TEST_CARGO_LOG="$cargo_log" \
        CODEX_INSTALL_TEST_DESTINATION="$destination" \
        "$launcher" install \
        > "$case_root/stdout" 2> "$case_root/stderr"
    local status=$?
    set -e

    # Assert
    if [[ "$status" -eq 0 ]]; then
        fail 'destination conflict: expected failure'
    fi
    assert_file_equals "$expected_file" "$destination" \
        'destination conflict: user entry changed'
    after_fingerprint="$(lstat_fingerprint "$destination")"
    if [[ "$after_fingerprint" != "$before_fingerprint" ]]; then
        fail 'destination conflict: entry identity, mode, or mtime changed'
    fi
    assert_absent "$cargo_log" 'destination conflict: Cargo must not run'
    assert_contains "$case_root/stderr" \
        "CONFLICT $destination" \
        'destination conflict: missing CONFLICT action'
}

test_restore_and_help_leave_link_unchanged() {
    # Arrange
    local case_root="$test_root/non-install"
    local case_home="$case_root/home"
    local fake_bin="$case_root/bin"
    local cargo_log="$case_root/cargo.log"
    local destination="$case_home/.local/bin/codex-upgrade"
    local unexpected_target="$case_root/unexpected-codex-upgrade"
    local before_fingerprint
    local after_fingerprint

    mkdir -p "$(dirname "$destination")" "$fake_bin"
    ln -s "$unexpected_target" "$destination"
    ln -s "$test_script" "$fake_bin/cargo"
    before_fingerprint="$(lstat_fingerprint "$destination")"

    # Act
    set +e
    HOME="$case_home" \
        PATH="$fake_bin:/usr/bin:/bin" \
        CODEX_INSTALL_TEST_FAKE_CARGO=1 \
        CODEX_INSTALL_TEST_CARGO_LOG="$cargo_log" \
        CODEX_INSTALL_TEST_DESTINATION="$destination" \
        "$launcher" restore --state-dir "$case_root/state" \
        > "$case_root/restore-stdout" 2> "$case_root/restore-stderr"
    local restore_status=$?
    HOME="$case_home" \
        PATH="$fake_bin:/usr/bin:/bin" \
        CODEX_INSTALL_TEST_FAKE_CARGO=1 \
        CODEX_INSTALL_TEST_CARGO_LOG="$cargo_log" \
        CODEX_INSTALL_TEST_DESTINATION="$destination" \
        "$launcher" --help \
        > "$case_root/help-stdout" 2> "$case_root/help-stderr"
    local help_status=$?
    set -e

    # Assert
    assert_status 0 "$restore_status" 'restore'
    assert_status 0 "$help_status" 'help'
    if [[ ! -L "$destination" ]] \
        || [[ "$(readlink "$destination")" != "$unexpected_target" ]]; then
        fail 'restore/help: unexpected helper symlink target changed'
    fi
    after_fingerprint="$(lstat_fingerprint "$destination")"
    if [[ "$after_fingerprint" != "$before_fingerprint" ]]; then
        fail 'restore/help: link identity, mode, or mtime changed'
    fi
    assert_cargo_call_count 2 "$cargo_log" 'restore/help'
}

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
test_script="$script_dir/install_test.sh"
launcher="$script_dir/../install.sh"
expected_source="$(cd "$script_dir/.." && pwd -P)/bin/codex-upgrade"
manifest_path="$(cd "$script_dir/../installer" && pwd -P)/Cargo.toml"
test_root="$(mktemp -d "${TMPDIR:-/tmp}/codex-install-test.XXXXXX")"
trap 'rm -rf "$test_root"' EXIT
trap 'exit 130' HUP INT TERM

test_successful_install_creates_link_after_rust
test_rust_failure_does_not_create_link
test_existing_exact_link_is_no_op
test_dry_run_reports_without_mutation
test_unexpected_destination_is_preserved_as_conflict
test_restore_and_help_leave_link_unchanged

printf '%s\n' 'PASS: Codex installer launcher tests'
