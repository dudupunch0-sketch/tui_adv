#!/usr/bin/env bash
# cloud_server_only
#
# Local-only helper for this cloud server (/home/dudupunch0/tui_adv).
# It keeps Rust toolchain/cache/build artifacts under /tmp so the small /home
# partition is not filled by rustup, cargo cache, or target/.
#
# Usage from project root:
#   ./cloud_server_only.sh install   # install missing Rust toolchain, build release smoke binary, refresh ./escape-terminal-cloud-server-only play launcher
#   ./cloud_server_only.sh run       # launch ./escape-terminal-cloud-server-only for direct interactive play
#   ./cloud_server_only.sh smoke     # run the Rust printer renderer smoke snapshot
#   ./cloud_server_only.sh test      # run Rust checks plus a Python direct-play smoke
#   ./cloud_server_only.sh env       # print the env exports for manual shell use
#
# Manual equivalent:
#   export RUSTUP_HOME=/tmp/dudupunch0-rustup
#   export CARGO_HOME=/tmp/dudupunch0-cargo
#   export CARGO_TARGET_DIR=/tmp/dudupunch0-tui-adv-target
#   export PATH="$CARGO_HOME/bin:$PATH"
#   rustup toolchain install stable --profile minimal --component rustfmt --component clippy
#   cargo build -p escape-terminal --release
#   ./cloud_server_only.sh install

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$REPO_ROOT"

export RUSTUP_HOME="${RUSTUP_HOME:-/tmp/dudupunch0-rustup}"
export CARGO_HOME="${CARGO_HOME:-/tmp/dudupunch0-cargo}"
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-/tmp/dudupunch0-tui-adv-target}"
export PATH="$CARGO_HOME/bin:$PATH"

TOOLCHAIN="${RUST_TOOLCHAIN:-stable}"
BIN_PATH="$CARGO_TARGET_DIR/release/escape-terminal"
LINK_PATH="$REPO_ROOT/escape-terminal-cloud-server-only"
PYTHON_BIN="${PYTHON_BIN:-python3}"

usage() {
  sed -n '1,32p' "$0" | sed 's/^# \{0,1\}//'
}

print_env() {
  cat <<EOF
export RUSTUP_HOME=$RUSTUP_HOME
export CARGO_HOME=$CARGO_HOME
export CARGO_TARGET_DIR=$CARGO_TARGET_DIR
export PATH=\"$CARGO_HOME/bin:\$PATH\"
EOF
}

ensure_dirs() {
  mkdir -p "$RUSTUP_HOME" "$CARGO_HOME" "$CARGO_TARGET_DIR"
}

ensure_rustup_command() {
  if command -v rustup >/dev/null 2>&1; then
    return 0
  fi

  cat >&2 <<EOF
rustup command was not found.

Fallback install command for this cloud server:
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | \
    RUSTUP_HOME=$RUSTUP_HOME CARGO_HOME=$CARGO_HOME \
    sh -s -- -y --profile minimal --default-toolchain $TOOLCHAIN

Then run:
  ./cloud_server_only.sh install
EOF
  exit 1
}

ensure_toolchain() {
  ensure_dirs
  ensure_rustup_command

  if rustup toolchain list | grep -q "^${TOOLCHAIN}-"; then
    return 0
  fi

  echo "Rust toolchain '$TOOLCHAIN' is missing under RUSTUP_HOME=$RUSTUP_HOME."
  echo "Installing fallback toolchain in /tmp so /home is not filled..."
  rustup toolchain install "$TOOLCHAIN" --profile minimal --component rustfmt --component clippy
}

write_play_launcher() {
  rm -f "$LINK_PATH"
  cat >"$LINK_PATH" <<'EOF_LAUNCHER'
#!/usr/bin/env bash
set -euo pipefail
REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PYTHON_BIN="${PYTHON_BIN:-python3}"
cd "$REPO_ROOT"
export PYTHONPATH="$REPO_ROOT/src${PYTHONPATH:+:$PYTHONPATH}"
exec "$PYTHON_BIN" -m tui_adv --play "$@"
EOF_LAUNCHER
  chmod +x "$LINK_PATH"
  echo "Wrote direct-play launcher: $LINK_PATH"
}

build_rust_binary() {
  ensure_toolchain
  cargo clean --release -p escape-terminal >/dev/null
  cargo build -p escape-terminal --release
}

build_release() {
  build_rust_binary
  write_play_launcher
  echo "Rust smoke binary: $BIN_PATH"
}

run_smoke() {
  if [[ ! -x "$BIN_PATH" ]]; then
    echo "Release binary is missing; running cloud-server-only fallback build first."
    build_release
  fi

  "$BIN_PATH" --scene printer --seed 123 --smoke
}

run_game() {
  if [[ ! -x "$LINK_PATH" ]]; then
    echo "Direct-play launcher is missing; creating it first."
    build_release
  fi

  exec "$LINK_PATH" "$@"
}

run_tests() {
  ensure_toolchain
  cargo fmt --check
  cargo test --workspace
  cargo clippy --workspace --all-targets -- -D warnings
  build_rust_binary
  "$BIN_PATH" --scene printer --seed 123 --smoke
  write_play_launcher
  "$PYTHON_BIN" -m pytest tests/test_cli.py::test_cli_play_mode_accepts_numbered_input_and_quit -q
  play_output="$(printf 'q\n' | "$LINK_PATH" --seed 123)"
  printf '%s\n' "$play_output" | grep -q "escape from the office - 직접 플레이"
}

command="${1:-install}"
if [[ $# -gt 0 ]]; then
  shift
fi

case "$command" in
  install)
    build_release
    ;;
  test)
    run_tests
    ;;
  run)
    run_game "$@"
    ;;
  smoke)
    run_smoke
    ;;
  env)
    print_env
    ;;
  help|--help|-h)
    usage
    ;;
  *)
    echo "unknown command: $command" >&2
    usage >&2
    exit 2
    ;;
esac
