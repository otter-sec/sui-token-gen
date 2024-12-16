# Manual Testing Checklist

## System-Level Tests
- [ ] Disk Space Test
  1. Fill disk space using `fallocate` (requires root)
  2. Attempt to create token
  3. Verify InsufficientDiskSpace error
  4. Clean up test files

- [ ] Permission Test
  1. Create folder with read-only permissions
  2. Attempt to create token in folder
  3. Verify PermissionDenied error
  4. Clean up test folder

- [ ] Concurrent Access Test
  1. Create .lock file manually
  2. Attempt to create token
  3. Verify ConcurrentAccess error
  4. Remove lock file
  5. Verify token can be created after lock removal

- [ ] Process Interruption Test
  1. Start token creation
  2. Interrupt process (Ctrl+C)
  3. Verify cleanup occurred
  4. Check no partial files remain

- [ ] Network Failure Test
  1. Start token creation
  2. Simulate network failure during RPC call
  3. Verify proper error handling
  4. Check cleanup occurred

## Test Commands
```bash
# Disk Space Test (requires root)
fallocate -l 10G /tmp/bigfile
# Create token and verify error
rm /tmp/bigfile

# Permission Test
mkdir test_token && chmod 444 test_token
# Create token and verify error
chmod 755 test_token && rm -rf test_token

# Concurrent Access Test
touch test_token.lock
# Create token and verify error
rm test_token.lock

# Process Interruption Test
# Start creation and press Ctrl+C during process

# Network Failure Test
# Use tc to simulate network issues during RPC call
sudo tc qdisc add dev lo root netem delay 1000ms loss 100%
# Create token and verify error
sudo tc qdisc del dev lo root
```

## Expected Results
1. All errors should be properly caught and reported
2. No partial files should remain after failures
3. System should maintain consistency
4. Error messages should be clear and actionable

## Notes
- Some tests require root permissions
- Network tests may require additional setup
- Document any unexpected behavior
- Note any system-specific requirements
