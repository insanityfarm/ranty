#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
WORKLOAD_DIR="$ROOT_DIR/benchmarks/workloads"
RESULTS_PATH="$ROOT_DIR/benchmarks/latest-results.json"
BINARY_PATH="${RANTY_BENCH_BINARY:-$ROOT_DIR/target/release/ranty}"

if ! command -v hyperfine >/dev/null 2>&1; then
  echo "hyperfine is required but was not found in PATH" >&2
  exit 1
fi

if [[ ! -x "$BINARY_PATH" ]]; then
  echo "Rust CLI binary not found at $BINARY_PATH" >&2
  echo "Run: cargo build --release --features cli --bin ranty" >&2
  exit 1
fi

rm -f "$RESULTS_PATH"

hyperfine \
  --style basic \
  --time-unit millisecond \
  --warmup 3 \
  --runs 20 \
  --input null \
  --output pipe \
  --export-json "$RESULTS_PATH" \
  -N \
  --command-name selector_ping_repeater \
  "$BINARY_PATH --no-debug --no-warnings --seed c0ffee $WORKLOAD_DIR/selector_ping_repeater.ranty" \
  --command-name temporal_labeled_cartesian_call \
  "$BINARY_PATH --no-debug --no-warnings --seed 1 $WORKLOAD_DIR/temporal_labeled_cartesian_call.ranty" \
  --command-name collection_callback_pipeline \
  "$BINARY_PATH --no-debug --no-warnings --seed 1 $WORKLOAD_DIR/collection_callback_pipeline.ranty" \
  --command-name unicode_reverse_repeater \
  "$BINARY_PATH --no-debug --no-warnings --seed 1 $WORKLOAD_DIR/unicode_reverse_repeater.ranty" \
  --command-name module_require_fanout \
  "$BINARY_PATH --no-debug --no-warnings --seed 1 $WORKLOAD_DIR/module_require_fanout.ranty"

echo "Results written to benchmarks/latest-results.json"
