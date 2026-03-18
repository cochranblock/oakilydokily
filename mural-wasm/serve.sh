#!/bin/bash
# Unlicense — cochranblock.org
# Contributors: Mattbusel (XFactor), GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3

# Serve mural-wasm standalone for local testing.
# Assets at /assets/* must resolve; run from mural-wasm/ so assets/ is at root.
cd "$(dirname "$0")"
echo "Serving mural-wasm at http://127.0.0.1:8765"
echo "Open http://127.0.0.1:8765/index.html"
python3 -m http.server 8765