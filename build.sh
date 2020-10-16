#!/bin/bash

set -e

echo -e "Building ibreq..."
cargo build --release -p ibreq
strip target/release/ibreq
du -h target/release/ibreq
echo -e "Done building ibreq.\n"

echo -e "Building keyrec..."
cargo build --release -p keyrec
strip target/release/keyrec
du -h target/release/keyrec
echo -e "Done building keyrec.\n"
