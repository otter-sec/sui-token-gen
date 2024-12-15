#!/bin/bash

# Start the RPC server in the background
cd rpc && cargo run --bin server -- --port 5000 &
SERVER_PID=$!

# Wait for server to start
sleep 2

# Create a test token
cd ..
export TOKEN_NAME="Test Token"
export TOKEN_SYMBOL="TEST"
export TOKEN_DECIMALS=6
export TOKEN_DESCRIPTION="A test token"
export TOKEN_FROZEN=false
export TOKEN_ENVIRONMENT="mainnet"

echo "Testing token creation..."
cargo run -- create

# Wait for token creation to complete
sleep 2

echo "Testing token verification..."
cargo run -- verify --path ./test_token

# Clean up
kill $SERVER_PID
