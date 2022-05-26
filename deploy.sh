#!/bin/sh

TARGET=pi@192.168.1.10

echo "Deployment target: $TARGET"

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
echo "Transferring executable..."
scp -o ControlPath=$SSH_SOCKET ./target/armv7-unknown-linux-gnueabihf/release/sprinkler $TARGET:~/sprinkler/sprinkler

# Transfer frontend using SCP.
echo "Transferring frontend files..."
ssh -q -o ControlPath=$SSH_SOCKET -t -p 22 $TARGET "rm -rf ~/sprinkler/frontend"
ssh -q -o ControlPath=$SSH_SOCKET -t -p 22 $TARGET "mkdir ~/sprinkler/frontend"
scp -o ControlPath=$SSH_SOCKET -r ./frontend $TARGET:~/sprinkler/

# Restarting service:
echo "Starting sprinkler service."
ssh -q -o ControlPath=$SSH_SOCKET -t -p 22 $TARGET "sudo systemctl start sprinkler"

# Done.
echo "Closing connection."

ssh -S $SSH_SOCKET -O exit $TARGET
echo "Deployment complete."