#!/bin/bash
cd "$(dirname "$0")"

if [ -f .server.pid ]; then
    kill $(cat .server.pid) 2>/dev/null
    rm .server.pid
    echo "Stopped"
else
    pkill -f llm-wiki-server 2>/dev/null && echo "Stopped" || echo "No server running"
fi
