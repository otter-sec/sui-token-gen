#!/bin/bash
set -e  # Exit on error

# Get the directory where the script is located
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cd "$SCRIPT_DIR"

# Function to cleanup RPC server
cleanup() {
    echo "Cleaning up RPC server..."
    # Kill any process using port 5000
    if command -v lsof >/dev/null 2>&1; then
        lsof -ti:5000 | xargs -r kill -9
    else
        fuser -k 5000/tcp >/dev/null 2>&1 || true
    fi
    # Kill our specific server process if it exists
    if [ ! -z "$SERVER_PID" ] && ps -p $SERVER_PID > /dev/null 2>&1; then
        kill $SERVER_PID
        wait $SERVER_PID 2>/dev/null || true
    fi
}

# Set up trap for cleanup on script exit
trap cleanup EXIT

# Check if RPC server is already running
if command -v lsof >/dev/null 2>&1; then
    if lsof -i:5000 >/dev/null 2>&1; then
        echo "Error: Port 5000 is already in use. Please stop any running RPC server first."
        exit 1
    fi
else
    if netstat -tuln 2>/dev/null | grep -q ":5000 "; then
        echo "Error: Port 5000 is already in use. Please stop any running RPC server first."
        exit 1
    fi
fi

echo "Starting RPC server..."
cd "$SCRIPT_DIR/rpc"
nohup cargo run --bin server -- --port 5000 > server.log 2>&1 & SERVER_PID=$!
if [ $? -ne 0 ]; then
    echo "Error: Failed to start RPC server"
    exit 1
fi

# Wait for server to start and verify it's running
for i in {1..10}; do
    if ! ps -p $SERVER_PID > /dev/null 2>&1; then
        echo "Error: RPC server process died"
        exit 1
    fi

    if (echo > /dev/tcp/127.0.0.1/5000) 2>/dev/null; then
        echo "RPC server is running and listening on port 5000"
        break
    fi

    if [ $i -eq 10 ]; then
        echo "Error: RPC server failed to bind to port 5000 after 10 attempts"
        exit 1
    fi

    echo "Waiting for RPC server to start (attempt $i/10)..."
    sleep 1
done

# Return to project root and set test environment variables
cd "$SCRIPT_DIR"
export TOKEN_NAME="Test Token"
export TOKEN_SYMBOL="TEST"
export TOKEN_DECIMALS=6
export TOKEN_DESCRIPTION="A test token"
export TOKEN_FROZEN=false
export TOKEN_ENVIRONMENT="mainnet"

echo "Testing token creation..."
if ! cargo run -- create; then
    echo "Error: Token creation failed"
    exit 1
fi

# Wait for token creation to complete
sleep 5

echo "Testing token verification..."
if ! cargo run -- verify --path "$SCRIPT_DIR/test_token"; then
    echo "Error: Token verification failed"
    exit 1
fi

# Cleanup is handled by the trap
