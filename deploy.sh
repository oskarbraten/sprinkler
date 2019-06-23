#!/bin/sh

TARGET=pi@sprinkler.local

echo "Building."
cargo build --release --target=armv7-unknown-linux-gnueabihf

# Setting up master connection (SSH).
SSH_SOCKET=~/.ssh/$TARGET
echo "Opening connection..."
ssh -M -f -N -o ControlPath=$SSH_SOCKET $TARGET

# Stopping service:
echo "Stopping sprinkler service."
ssh -q -o ControlPath=$SSH_SOCKET -t -p 22 $TARGET "sudo systemctl stop sprinkler"

# Transfer executable using SCP.
echo "Deploying executable."
scp -o ControlPath=$SSH_SOCKET ./target/armv7-unknown-linux-gnueabihf/release/sprinkler $TARGET:~/sprinkler/sprinkler

# Restarting service:
echo "Starting sprinkler service."
ssh -q -o ControlPath=$SSH_SOCKET -t -p 22 $TARGET "sudo systemctl start sprinkler"

# Done.
echo "Closing connection."

ssh -S $SSH_SOCKET -O exit $TARGET
echo "Deployment complete."