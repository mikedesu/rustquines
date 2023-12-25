#!/usr/bin/zsh

./generate_quine.sh > src/main.rs

while true; do 
    cat src/main.rs;  
    cargo build;  
    ./target/debug/quines > src/main.rs; 
    sleep 1; 
done
