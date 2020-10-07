#!/bin/bash

cargo build --release
strip target/release/ibreq
du -h target/release/ibreq