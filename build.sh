#!/bin/bash

set -e

cargo build --release -p ibreq
strip target/release/ibreq
du -h target/release/ibreq