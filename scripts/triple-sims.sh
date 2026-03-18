#!/bin/bash
# Unlicense — cochranblock.org
# Contributors: Mattbusel (XFactor), GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3

# Run TRIPLE SIMS: 3 sequential passes. All must pass.
# Use: ./scripts/triple-sims.sh [--http]
# --http: use sim-http.sh against BASE (server must be up) instead of cargo test

set -e
cd "$(dirname "$0")/.."
ROOT="$(pwd)"

if [[ "$1" = "--http" ]]; then
  BASE="${BASE:-http://127.0.0.1:3000}"
  echo "TRIPLE SIMS (HTTP) against $BASE"
  for i in 1 2 3; do
    echo "--- Pass $i ---"
    ./scripts/sim-http.sh || exit 1
  done
  echo "All 3 passes OK"
  exit 0
fi

# Cargo-based (when oakilydokily-test exists)
echo "TRIPLE SIMS (cargo)"
for i in 1 2 3; do
  echo "--- Pass $i ---"
  cargo run --bin oakilydokily-test --features tests -- --test || exit 1
done
echo "All 3 passes OK"
exit 0