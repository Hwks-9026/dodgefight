#!/bin/bash
cd standalone
cargo build -r
mv target/release/standalone ../builds/