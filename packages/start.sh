#!/bin/bash
cd "$(dirname "$0")"

export DATA_DIR="${DATA_DIR:-/tmp/llm_wiki_data}"
mkdir -p "$DATA_DIR"

BINARY="./server/target/release/llm-wiki-server"

if [ ! -f "$BINARY" ]; then
    echo "Binary not found. Run ./build.sh first."
    exit 1
fi

echo "Starting LLM Wiki..."
echo "DATA_DIR: $DATA_DIR"

$BINARY &
echo $! > .server.pid

echo ""
echo "Server: http://localhost:3001"
echo ""
echo "Run ./stop.sh to stop the server"
