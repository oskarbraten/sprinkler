#!/bin/sh
cargo build --release --target=armv7-unknown-linux-gnueabihf
scp ./target/armv7-unknown-linux-gnueabihf/release/sprinkler pi@sprinkler.local:~/sprinkler