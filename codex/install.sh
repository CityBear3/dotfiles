#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
manifest_path="$script_dir/installer/Cargo.toml"

exec cargo run --quiet --locked --release --manifest-path "$manifest_path" -- "$@"
