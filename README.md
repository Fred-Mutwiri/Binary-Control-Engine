# Binary Control Engine #

This is an educational Rust Project exploring how binary data can become instructions, machine state and control flow.

This project implements a small experimental instruction machine from scratch, including:

- bit manipulation
- binary encoding and decoding
- arithmetics with bit-level algorithms
- packed flags
- registers and memori
- program counter
- instructions fetch, decode and execute
- conditional branching
- Custom macro-based iteration
- checksums
- binary program format


These mechanisms are under study, therefore NO serialization framework or VM framework mechanisms are used.

The goal is to understand this relationship:

bits => representation => instructions => state => control flow => execution


# Why build this? #
As previously stated that no serialization framework or VM framework will be used, this project is part of my systems-programming journey.
I am using it to understand the binary representation, state, control flow, encoding and execution before moving into larger systems.

obviously this is step by step


# How to run:
// This runs the program 

cargo run


// This runs the tests in the program

cargo test