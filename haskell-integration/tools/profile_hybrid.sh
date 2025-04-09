#!/bin/bash
# filepath: c:\Users\Tim\Desktop\LMS\haskell-integration\tools\profile_hybrid.sh

# Ensure we're in the project root
cd "$(dirname "$0")/.."

# Capture baseline metrics
echo "Capturing baseline metrics..."
cargo bench --bench sync_benchmark -- --profile-time=10 > rust_baseline.txt

# Run integrated profiling
echo "Running integrated profiling..."
GHCRTS="+RTS -p -h -l -xt -RTS" \
cargo bench --bench hybrid_benchmark -- --profile-time=10

# Generate flame graph (requires FlameGraph tools in PATH)
echo "Generating flame graph..."
perf record -F 99 -g -- cargo bench --bench hybrid_benchmark -- --profile-time=5
perf script | stackcollapse-perf.pl | flamegraph.pl > hybrid_flame.svg

# Analyze GC behavior
echo "Analyzing GC behavior..."
hp2ps -e8in -c *.hp

echo "Profiling complete. Results available in:"
echo "- hybrid_flame.svg (flame graph)"
echo "- *.hp.ps (heap profile)"
echo "- rust_baseline.txt (baseline metrics)"