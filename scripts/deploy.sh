#!/bin/bash

# Build the Solana program
cargo build-bpf

# Deploy the program
solana program deploy target/deploy/sparta_x.so
