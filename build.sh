#!/bin/bash

set -e

echo -e "Building ibreq..."
export CONF_PATH="//"
export CONF_METHOD="GET"

cargo run --release -p ibreq
cargo build --release -p ibreq
#strip target/release/ibreq
#du -h target/release/ibreq
echo -e "Done building ibreq.\n"

echo -e "Building keyrec..."
export CONF_PATH="//keys"
export CONF_METHOD="POST"

cargo build --release -p keyrec
#strip target/release/keyrec
#du -h target/release/keyrec
echo -e "Done building keyrec.\n"
