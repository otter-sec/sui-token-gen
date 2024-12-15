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

# Build the server first
echo "Building RPC server..."
if ! cargo build --bin server; then
    echo "Error: Failed to build RPC server"
    exit 1
fi

# Start the server with logging
echo "Starting RPC server process..."
RUST_BACKTRACE=1 nohup cargo run --bin server -- --port 5000 > server.log 2>&1 & SERVER_PID=$!

# Function to check server connection
check_server_connection() {
    local max_retries=5
    local retry_count=0
    local connected=false

    while [ $retry_count -lt $max_retries ]; do
        if (echo > /dev/tcp/127.0.0.1/5000) 2>/dev/null; then
            connected=true
            break
        fi
        retry_count=$((retry_count + 1))
        sleep 1
    done

    echo $connected
}

# Wait for server to start and verify it's running
for i in {1..60}; do
    if ! ps -p $SERVER_PID > /dev/null 2>&1; then
        echo "Error: RPC server process died"
        echo "Server log:"
        cat server.log
        exit 1
    fi

    connection_status=$(check_server_connection)
    if [ "$connection_status" = "true" ]; then
        echo "RPC server is running and listening on port 5000"
        # Add extra wait time to ensure server is fully ready
        sleep 5
        break
    fi

    if [ $i -eq 60 ]; then
        echo "Error: RPC server failed to bind to port 5000 after 60 attempts"
        echo "Server log:"
        cat server.log
        exit 1
    fi

    echo "Waiting for RPC server to start (attempt $i/60)..."
    sleep 2
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
# Verify server is still running before token creation
if ! $(check_server_connection); then
    echo "Error: RPC server is not responding before token creation"
    cat server.log
    exit 1
fi

if ! cargo run -- create; then
    echo "Error: Token creation failed"
    cat server.log
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
