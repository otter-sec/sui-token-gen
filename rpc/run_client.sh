#!/bin/bash

# Define the command
COMMAND="cargo run --bin client -- --server-addr [::1]:5000 --command verify_url --url https://gitlab.com/osec/test-sui-token"

# Loop to execute the command 500 times in parallel
for i in {1..500}
do
  echo "Starting command $i/500"
  $COMMAND &
done

# Wait for all background processes to complete
wait

echo "All 500 commands executed in parallel."
