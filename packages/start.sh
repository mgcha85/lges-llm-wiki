#!/bin/bash
cd "$(dirname "$0")"

export DATA_DIR="${DATA_DIR:-/tmp/llm_wiki_data}"
mkdir -p "$DATA_DIR"

BINARY="./server/target/release/llm-wiki-server"
LOG_FILE="./.server.log"

if [ ! -f "$BINARY" ]; then
    echo "Binary not found. Run ./build.sh first."
    exit 1
fi

echo "Starting LLM Wiki..."
echo "DATA_DIR: $DATA_DIR"
echo "LOG_FILE: $LOG_FILE"

nohup "$BINARY" > "$LOG_FILE" 2>&1 &
echo $! > .server.pid

echo ""
echo "Server: http://localhost:3001"
echo "Logs: tail -f $LOG_FILE"
echo ""
echo "Run ./stop.sh to stop the server"
