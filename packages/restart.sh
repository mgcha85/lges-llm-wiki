#!/bin/bash
set -euo pipefail
cd "$(dirname "$0")"

echo "=========================================="
echo "LLM Wiki Restart Pipeline"
echo "=========================================="

echo ""
echo "[1/4] Stopping existing server (if any)..."
bash ./stop.sh || true

echo ""
echo "[2/4] Building project..."
bash ./build.sh

echo ""
echo "[3/4] Starting server..."
bash ./start.sh

echo ""
echo "[4/4] Health check..."
if curl --retry 20 --retry-delay 1 --retry-connrefused --max-time 5 -fsS http://localhost:3001 > /dev/null; then
    echo "Health check passed: http://localhost:3001"
else
    echo "Health check failed"
    exit 1
fi

echo ""
echo "=========================================="
echo "Pipeline complete"
echo "=========================================="
