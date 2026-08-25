#!/bin/sh
set -eu

run_fake_codex() {
    case $# in
        1)
            if [ "$1" = update ]; then
                canonical_call=update
            else
                printf 'unexpected codex invocation (%s args): %s\n' "$#" "$*" >&2
                exit 99
            fi
            ;;
        3)
            if [ "$1" = app-server ] && [ "$2" = daemon ] && [ "$3" = version ]; then
                canonical_call='app-server daemon version'
            elif [ "$1" = app-server ] && [ "$2" = daemon ] && [ "$3" = restart ]; then
                canonical_call='app-server daemon restart'
            else
                printf 'unexpected codex invocation (%s args): %s\n' "$#" "$*" >&2
                exit 99
            fi
            ;;
        *)
            printf 'unexpected codex invocation (%s args): %s\n' "$#" "$*" >&2
            exit 99
            ;;
    esac

    printf '%s\n' "$canonical_call" >> "$CODEX_TEST_CALL_LOG"

    case "$canonical_call" in
        'app-server daemon version')
            if [ -f "$CODEX_TEST_VERSION_SEEN" ]; then
                printf '%s\n' 'post-version-output'
                printf '%s\n' 'post-version-error' >&2
                exit "${CODEX_TEST_POST_VERSION_STATUS:-0}"
            fi

            : > "$CODEX_TEST_VERSION_SEEN"
            printf '%s\n' 'pre-version-output'
            printf '%s\n' 'pre-version-error' >&2
            exit "${CODEX_TEST_PRE_VERSION_STATUS:-0}"
            ;;
        update)
            printf '%s\n' 'update-output'
            printf '%s\n' 'update-error' >&2
            exit "${CODEX_TEST_UPDATE_STATUS:-0}"
            ;;
        'app-server daemon restart')
            printf '%s\n' 'restart-output'
            printf '%s\n' 'restart-error' >&2
            exit "${CODEX_TEST_RESTART_STATUS:-0}"
            ;;
    esac
}

if [ "${CODEX_UPGRADE_TEST_FAKE:-0}" = 1 ]; then
    run_fake_codex "$@"
fi

unset CODEX_UPGRADE_TEST_FAKE \
    CODEX_TEST_CALL_LOG \
    CODEX_TEST_VERSION_SEEN \
    CODEX_TEST_PRE_VERSION_STATUS \
    CODEX_TEST_UPDATE_STATUS \
    CODEX_TEST_RESTART_STATUS \
    CODEX_TEST_POST_VERSION_STATUS

fail() {
    printf 'FAIL: %s\n' "$1" >&2
    exit 1
}

assert_status() {
    expected_status=$1
    actual_status=$2
    label=$3

    if [ "$actual_status" -ne "$expected_status" ]; then
        fail "$label: expected status $expected_status, got $actual_status"
    fi
}

assert_file_equals() {
    expected_file=$1
    actual_file=$2
    label=$3

    if ! cmp -s "$expected_file" "$actual_file"; then
        printf 'FAIL: %s\n' "$label" >&2
        diff -u "$expected_file" "$actual_file" >&2 || :
        exit 1
    fi
}

arrange_case() {
    case_name=$1
    case_root="$test_root/$case_name"
    fake_bin="$case_root/bin"
    call_log="$case_root/calls.log"
    version_seen="$case_root/version-seen"
    stdout_file="$case_root/stdout"
    stderr_file="$case_root/stderr"
    expected_file="$case_root/expected"

    mkdir -p "$fake_bin"
    ln -s "$test_script" "$fake_bin/codex"
}

test_fake_rejects_collapsed_argv() {
    # Arrange
    arrange_case collapsed-argv

    # Act
    set +e
    PATH="$fake_bin:/usr/bin:/bin" \
        CODEX_UPGRADE_TEST_FAKE=1 \
        CODEX_TEST_CALL_LOG="$call_log" \
        CODEX_TEST_VERSION_SEEN="$version_seen" \
        "$fake_bin/codex" 'app-server daemon version' \
        > "$stdout_file" 2> "$stderr_file"
    status=$?
    set -e

    # Assert
    assert_status 99 "$status" 'collapsed argv'
    if [ -e "$call_log" ]; then
        fail 'collapsed argv: unexpected canonical call log'
    fi
    : > "$expected_file"
    assert_file_equals "$expected_file" "$stdout_file" 'collapsed argv stdout'
    printf '%s\n' \
        'unexpected codex invocation (1 args): app-server daemon version' \
        > "$expected_file"
    assert_file_equals "$expected_file" "$stderr_file" 'collapsed argv stderr'
}

test_running_daemon_is_restarted_after_update() {
    # Arrange
    arrange_case running-daemon

    # Act
    set +e
    PATH="$fake_bin:/usr/bin:/bin" \
        CODEX_UPGRADE_TEST_FAKE=1 \
        CODEX_TEST_CALL_LOG="$call_log" \
        CODEX_TEST_VERSION_SEEN="$version_seen" \
        "$helper" > "$stdout_file" 2> "$stderr_file"
    status=$?
    set -e

    # Assert
    assert_status 0 "$status" 'running daemon'
    printf '%s\n' \
        'app-server daemon version' \
        'update' \
        'app-server daemon restart' \
        'app-server daemon version' > "$expected_file"
    assert_file_equals "$expected_file" "$call_log" 'running daemon call order'
    printf '%s\n' \
        'update-output' \
        'restart-output' \
        'post-version-output' > "$expected_file"
    assert_file_equals "$expected_file" "$stdout_file" 'running daemon stdout'
    printf '%s\n' \
        'update-error' \
        'restart-error' \
        'post-version-error' > "$expected_file"
    assert_file_equals "$expected_file" "$stderr_file" 'running daemon stderr'
}

test_stopped_daemon_is_not_started() {
    # Arrange
    arrange_case stopped-daemon

    # Act
    set +e
    PATH="$fake_bin:/usr/bin:/bin" \
        CODEX_UPGRADE_TEST_FAKE=1 \
        CODEX_TEST_CALL_LOG="$call_log" \
        CODEX_TEST_VERSION_SEEN="$version_seen" \
        CODEX_TEST_PRE_VERSION_STATUS=17 \
        "$helper" > "$stdout_file" 2> "$stderr_file"
    status=$?
    set -e

    # Assert
    assert_status 0 "$status" 'stopped daemon'
    printf '%s\n' \
        'app-server daemon version' \
        'update' > "$expected_file"
    assert_file_equals "$expected_file" "$call_log" 'stopped daemon call order'
    printf '%s\n' \
        'update-output' \
        'Codex updated; app-server daemon was not running.' > "$expected_file"
    assert_file_equals "$expected_file" "$stdout_file" 'stopped daemon stdout'
    printf '%s\n' 'update-error' > "$expected_file"
    assert_file_equals "$expected_file" "$stderr_file" 'stopped daemon stderr'
}

test_update_failure_stops_before_restart() {
    # Arrange
    arrange_case update-failure

    # Act
    set +e
    PATH="$fake_bin:/usr/bin:/bin" \
        CODEX_UPGRADE_TEST_FAKE=1 \
        CODEX_TEST_CALL_LOG="$call_log" \
        CODEX_TEST_VERSION_SEEN="$version_seen" \
        CODEX_TEST_UPDATE_STATUS=23 \
        "$helper" > "$stdout_file" 2> "$stderr_file"
    status=$?
    set -e

    # Assert
    assert_status 23 "$status" 'update failure'
    printf '%s\n' \
        'app-server daemon version' \
        'update' > "$expected_file"
    assert_file_equals "$expected_file" "$call_log" 'update failure call order'
    printf '%s\n' 'update-output' > "$expected_file"
    assert_file_equals "$expected_file" "$stdout_file" 'update failure stdout'
    printf '%s\n' 'update-error' > "$expected_file"
    assert_file_equals "$expected_file" "$stderr_file" 'update failure stderr'
}

test_stopped_daemon_update_failure_is_propagated() {
    # Arrange
    arrange_case stopped-daemon-update-failure

    # Act
    set +e
    PATH="$fake_bin:/usr/bin:/bin" \
        CODEX_UPGRADE_TEST_FAKE=1 \
        CODEX_TEST_CALL_LOG="$call_log" \
        CODEX_TEST_VERSION_SEEN="$version_seen" \
        CODEX_TEST_PRE_VERSION_STATUS=17 \
        CODEX_TEST_UPDATE_STATUS=23 \
        "$helper" > "$stdout_file" 2> "$stderr_file"
    status=$?
    set -e

    # Assert
    assert_status 23 "$status" 'stopped daemon update failure'
    printf '%s\n' \
        'app-server daemon version' \
        'update' > "$expected_file"
    assert_file_equals "$expected_file" "$call_log" \
        'stopped daemon update failure call order'
    printf '%s\n' 'update-output' > "$expected_file"
    assert_file_equals "$expected_file" "$stdout_file" \
        'stopped daemon update failure stdout'
    printf '%s\n' 'update-error' > "$expected_file"
    assert_file_equals "$expected_file" "$stderr_file" \
        'stopped daemon update failure stderr'
}

test_restart_failure_is_propagated() {
    # Arrange
    arrange_case restart-failure

    # Act
    set +e
    PATH="$fake_bin:/usr/bin:/bin" \
        CODEX_UPGRADE_TEST_FAKE=1 \
        CODEX_TEST_CALL_LOG="$call_log" \
        CODEX_TEST_VERSION_SEEN="$version_seen" \
        CODEX_TEST_RESTART_STATUS=31 \
        "$helper" > "$stdout_file" 2> "$stderr_file"
    status=$?
    set -e

    # Assert
    assert_status 31 "$status" 'restart failure'
    printf '%s\n' \
        'app-server daemon version' \
        'update' \
        'app-server daemon restart' > "$expected_file"
    assert_file_equals "$expected_file" "$call_log" 'restart failure call order'
    printf '%s\n' \
        'update-output' \
        'restart-output' > "$expected_file"
    assert_file_equals "$expected_file" "$stdout_file" 'restart failure stdout'
    printf '%s\n' \
        'update-error' \
        'restart-error' > "$expected_file"
    assert_file_equals "$expected_file" "$stderr_file" 'restart failure stderr'
}

test_post_restart_version_failure_is_propagated() {
    # Arrange
    arrange_case post-version-failure

    # Act
    set +e
    PATH="$fake_bin:/usr/bin:/bin" \
        CODEX_UPGRADE_TEST_FAKE=1 \
        CODEX_TEST_CALL_LOG="$call_log" \
        CODEX_TEST_VERSION_SEEN="$version_seen" \
        CODEX_TEST_POST_VERSION_STATUS=41 \
        "$helper" > "$stdout_file" 2> "$stderr_file"
    status=$?
    set -e

    # Assert
    assert_status 41 "$status" 'post-restart version failure'
    printf '%s\n' \
        'app-server daemon version' \
        'update' \
        'app-server daemon restart' \
        'app-server daemon version' > "$expected_file"
    assert_file_equals "$expected_file" "$call_log" 'post-restart version failure call order'
    printf '%s\n' \
        'update-output' \
        'restart-output' \
        'post-version-output' > "$expected_file"
    assert_file_equals "$expected_file" "$stdout_file" 'post-restart version failure stdout'
    printf '%s\n' \
        'update-error' \
        'restart-error' \
        'post-version-error' > "$expected_file"
    assert_file_equals "$expected_file" "$stderr_file" 'post-restart version failure stderr'
}

script_dir=$(CDPATH= cd "$(dirname "$0")" && pwd)
test_script="$script_dir/codex-upgrade_test.sh"
helper="$script_dir/../bin/codex-upgrade"
test_root=$(mktemp -d "${TMPDIR:-/tmp}/codex-upgrade-test.XXXXXX")
trap 'rm -rf "$test_root"' 0
trap 'exit 130' 1 2 15

test_fake_rejects_collapsed_argv
test_running_daemon_is_restarted_after_update
test_stopped_daemon_is_not_started
test_update_failure_stops_before_restart
test_stopped_daemon_update_failure_is_propagated
test_restart_failure_is_propagated
test_post_restart_version_failure_is_propagated

printf '%s\n' 'PASS: codex-upgrade black-box tests'
