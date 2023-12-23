#!/usr/bin/zsh

python3 makequine.py template.rs > test.rs && rustc test.rs && ./test;
