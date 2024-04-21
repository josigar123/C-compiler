#!/bin/bash

# Compile and run the program
cargo build

cargo run

# Assemble the assembly
as -o bin/out.o bin/out.s

# Use linker to create executable
ld -o bin/out bin/out.o

# Run the program
./bin/out
