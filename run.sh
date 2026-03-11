# Unlicense — cochranblock.org
# Contributors: GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3
#!/bin/bash
# Run oakilydokily. Keeps running until Ctrl+C.
cd "$(dirname "$0")"
pkill -f "oakilydokily" 2>/dev/null && sleep 1
exec ./target/release/oakilydokily
