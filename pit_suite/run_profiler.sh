#!/usr/bin/env bash
# run_profiler.sh — run ssip fusion profiler over all SPEC2006 PIT traces
# Usage: ./run_profiler.sh [OUTPUT_DIR]
# Output dir defaults to ./results/<timestamp>

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SSIP_DIR="$SCRIPT_DIR/ssip"
TRACE_ROOT="/fast-lab-share/pradyun/SPEC2K6_TRACES"
OUTPUT_DIR="${1:-$SCRIPT_DIR/results/$(date +%Y%m%d_%H%M%S)}"
BINARY="$SSIP_DIR/target/release/ssip"
JOBS="${JOBS:-$(nproc)}"

# ── build ────────────────────────────────────────────────────────────────────
echo "[build] compiling ssip (release)..."
cargo build --release --manifest-path "$SSIP_DIR/Cargo.toml" 2>&1 \
    | sed 's/^/  /'
echo "[build] done: $BINARY"

# ── output layout ────────────────────────────────────────────────────────────
mkdir -p "$OUTPUT_DIR"
echo "[info] writing results to $OUTPUT_DIR"
echo "[info] parallelism: $JOBS jobs"

# ── worker function ──────────────────────────────────────────────────────────
run_one() {
    local pit="$1"
    local benchmark
    benchmark="$(basename "$(dirname "$pit")")"
    local trace_name
    trace_name="$(basename "$pit" .pit)"

    local out_dir="$OUTPUT_DIR/$benchmark"
    mkdir -p "$out_dir"
    local out_file="$out_dir/${trace_name}.txt"

    if "$BINARY" "$pit" > "$out_file" 2>&1; then
        echo "[ok]   $benchmark/$trace_name"
    else
        echo "[fail] $benchmark/$trace_name (exit $?)" >&2
    fi
}
export -f run_one
export BINARY OUTPUT_DIR

# ── run ──────────────────────────────────────────────────────────────────────
total=$(find "$TRACE_ROOT" -name '*.pit' | wc -l)
echo "[info] found $total traces across $(ls "$TRACE_ROOT" | wc -l) benchmarks"

find "$TRACE_ROOT" -name '*.pit' \
    | sort \
    | xargs -P "$JOBS" -I{} bash -c 'run_one "$@"' _ {}

echo "[done] results in $OUTPUT_DIR"
