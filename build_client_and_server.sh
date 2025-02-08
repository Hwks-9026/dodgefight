#!/bin/bash
cd client
cargo build -r
mv target/release/client ../builds/
cd ..
cd server
cargo build -r
mv target/release/server ../builds/
cd ..
