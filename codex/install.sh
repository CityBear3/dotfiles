#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
manifest_path="$script_dir/installer/Cargo.toml"
helper_action=
helper_invocation=passthrough

is_valid_agent_threads() {
    case "$1" in
        auto | [2-9] | [12][0-9] | 3[0-2])
            return 0
            ;;
        *)
            return 1
            ;;
    esac
}

classify_invocation() {
    local argument
    local expected_value=
    local seen_adopt_existing=0
    local seen_agent_threads=0
    local seen_codex_home=0
    local seen_dry_run=0
    local seen_skills_home=0
    local seen_state_dir=0
    local value

    for argument in "$@"; do
        if [[ "$argument" == -h || "$argument" == --help ]]; then
            return
        fi
    done

    if [[ $# -eq 0 ]]; then
        helper_invocation=install
        return
    fi

    case "$1" in
        restore)
            return
            ;;
        install)
            shift
            ;;
        -*)
            ;;
        *)
            return
            ;;
    esac

    for argument in "$@"; do
        if [[ -n "$expected_value" ]]; then
            if [[ "$argument" == -* ]]; then
                helper_invocation=passthrough
                return
            fi
            if [[ "$expected_value" == agent-threads ]] \
                && ! is_valid_agent_threads "$argument"; then
                helper_invocation=passthrough
                return
            fi
            expected_value=
            continue
        fi

        case "$argument" in
            --dry-run)
                if [[ "$seen_dry_run" -eq 1 ]]; then
                    helper_invocation=passthrough
                    return
                fi
                seen_dry_run=1
                helper_invocation=dry-run
                ;;
            --adopt-existing)
                if [[ "$seen_adopt_existing" -eq 1 ]]; then
                    helper_invocation=passthrough
                    return
                fi
                seen_adopt_existing=1
                ;;
            --agent-threads)
                if [[ "$seen_agent_threads" -eq 1 ]]; then
                    helper_invocation=passthrough
                    return
                fi
                seen_agent_threads=1
                expected_value=agent-threads
                ;;
            --codex-home)
                if [[ "$seen_codex_home" -eq 1 ]]; then
                    helper_invocation=passthrough
                    return
                fi
                seen_codex_home=1
                expected_value=path
                ;;
            --agent-threads=*)
                if [[ "$seen_agent_threads" -eq 1 ]]; then
                    helper_invocation=passthrough
                    return
                fi
                seen_agent_threads=1
                value=${argument#--agent-threads=}
                if ! is_valid_agent_threads "$value"; then
                    helper_invocation=passthrough
                    return
                fi
                ;;
            --codex-home=*)
                if [[ "$seen_codex_home" -eq 1 ]]; then
                    helper_invocation=passthrough
                    return
                fi
                seen_codex_home=1
                ;;
            --skills-home)
                if [[ "$seen_skills_home" -eq 1 ]]; then
                    helper_invocation=passthrough
                    return
                fi
                seen_skills_home=1
                expected_value=path
                ;;
            --skills-home=*)
                if [[ "$seen_skills_home" -eq 1 ]]; then
                    helper_invocation=passthrough
                    return
                fi
                seen_skills_home=1
                ;;
            --state-dir)
                if [[ "$seen_state_dir" -eq 1 ]]; then
                    helper_invocation=passthrough
                    return
                fi
                seen_state_dir=1
                expected_value=path
                ;;
            --state-dir=*)
                if [[ "$seen_state_dir" -eq 1 ]]; then
                    helper_invocation=passthrough
                    return
                fi
                seen_state_dir=1
                ;;
            *)
                helper_invocation=passthrough
                return
                ;;
        esac
    done

    if [[ -n "$expected_value" ]]; then
        helper_invocation=passthrough
        return
    fi

    if [[ "$helper_invocation" != dry-run ]]; then
        helper_invocation=install
    fi
}

preflight_helper_link() {
    if [[ ! -f "$helper_source" || -L "$helper_source" || ! -x "$helper_source" ]]; then
        printf 'Codex upgrade helper source is not an executable regular file: %s\n' \
            "$helper_source" >&2
        return 1
    fi

    if [[ -L "$helper_destination" ]]; then
        if [[ "$(readlink "$helper_destination")" == "$helper_source" ]]; then
            helper_action=NO-OP
            printf 'NO-OP %s -> %s\n' "$helper_destination" "$helper_source"
            return
        fi

        helper_action=CONFLICT
        printf 'CONFLICT %s: expected symlink target %s\n' \
            "$helper_destination" "$helper_source" >&2
        return 1
    fi

    if [[ -e "$helper_destination" ]]; then
        helper_action=CONFLICT
        printf 'CONFLICT %s: destination already exists\n' \
            "$helper_destination" >&2
        return 1
    fi

    helper_action=CREATE
    printf 'CREATE %s -> %s\n' "$helper_destination" "$helper_source"
}

run_rust_installer() {
    cargo run --quiet --locked --release --manifest-path "$manifest_path" -- "$@"
}

create_helper_link() {
    if ! mkdir -p "$helper_directory"; then
        printf 'Failed to create Codex helper directory: %s\n' \
            "$helper_directory" >&2
        return 1
    fi

    if [[ -e "$helper_destination" || -L "$helper_destination" ]]; then
        printf 'CONFLICT %s: destination appeared after installer success\n' \
            "$helper_destination" >&2
        return 1
    fi

    # Supplying the parent as ln's directory operand makes it create the fixed
    # source basename exclusively. A raced destination is preserved as EEXIST,
    # including when that entry is itself a directory.
    if ! ln -s "$helper_source" "$helper_directory"; then
        printf 'Failed to create Codex upgrade helper link: %s\n' \
            "$helper_destination" >&2
        return 1
    fi
}

classify_invocation "$@"

if [[ "$helper_invocation" == passthrough ]]; then
    exec cargo run --quiet --locked --release --manifest-path "$manifest_path" -- "$@"
fi

if [[ -z "${HOME:-}" ]]; then
    printf 'HOME is required to locate the Codex upgrade helper link\n' >&2
    exit 1
fi

helper_source="$script_dir/bin/codex-upgrade"
helper_local_directory="$HOME/.local"
helper_directory="$helper_local_directory/bin"
helper_destination="$helper_directory/codex-upgrade"

if [[ ( -e "$helper_local_directory" || -L "$helper_local_directory" ) \
    && ! -d "$helper_local_directory" ]]; then
    printf 'CONFLICT %s: helper parent is not a directory\n' \
        "$helper_local_directory" >&2
    exit 1
fi

if [[ ( -e "$helper_directory" || -L "$helper_directory" ) \
    && ! -d "$helper_directory" ]]; then
    printf 'CONFLICT %s: helper parent is not a directory\n' \
        "$helper_directory" >&2
    exit 1
fi

preflight_helper_link

if run_rust_installer "$@"; then
    :
else
    rust_status=$?
    exit "$rust_status"
fi

if [[ "$helper_invocation" == dry-run || "$helper_action" == NO-OP ]]; then
    exit 0
fi

create_helper_link
