#!/bin/bash
set -e
cd "$(dirname "$0")"

echo "=========================================="
echo "Building LLM Wiki"
echo "=========================================="

echo ""
echo "[1/3] Installing frontend dependencies..."
cd web
npm install

echo ""
echo "[2/3] Building frontend..."
npm run build

echo ""
echo "[3/3] Building backend (with embedded frontend)..."
cd ../server
cargo build --release

echo ""
echo "=========================================="
echo "Build complete!"
echo "Binary: packages/server/target/release/llm-wiki-server"
echo ""
echo "Run ./start.sh to start the server"
echo "=========================================="
