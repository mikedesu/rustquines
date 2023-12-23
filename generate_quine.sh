#!/usr/bin/zsh

python3 makequine.py template.rs > test.rs;
cp test.rs src/main.rs;
cargo build;
./target/debug/quines;
