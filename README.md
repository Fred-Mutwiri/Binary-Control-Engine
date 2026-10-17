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


# Where this is right now

The idea is still the same: start from bits and grow a small machine without borrowing a VM or a serializer. A lot of the layers exist. They are not all talking to each other yet.

`cargo run` walks three things: bit ops on `0b10110110`, the arithmetic engine, and a tiny program `LOAD 5 / ADD 3 / SUB 2 / HALT`. That program ends with 6 in the accumulator.


## What is actually working

**Bits.** Get, set, clear, toggle. Single-bit masks and field masks. Extract a field, count set bits, isolate the lowest set bit, reverse, rotate. Notes for some of this live under `Docs/bits-doc/`.

**Arithmetic.** Built from a half adder and a full adder, not from `+` / `-`. Add, subtract (two's complement), increment, decrement, compare, shift-and-add multiply (keeps a high byte), restoring divide. Wrappers return the value plus carry or borrow.

**Flags.** Zero, carry, borrow, negative, overflow. Packed constants for those bits. `add_flags` fills a `Flags` struct after an add.

**CPU, as far as it goes.** Each instruction byte is a 4-bit opcode and a 4-bit operand.

```
0000  NOP
0001  LOAD
0010  ADD
0011  SUB
0100  JUMP
0101  JUMP_IF_ZERO
1111  HALT
```

Fetch, decode (`extract_field`), execute. The program counter advances or jumps. HALT stops the loop. Flags get updated on load/add/sub. This path uses a bare `u8` accumulator and `src/cpu/memory.rs` as the program bytes.

**Control extras.** A counter that increments/decrements through the arithmetic engine, plus `count_up` / `count_down`.


## Built, but not in the running machine yet

These are real types with tests. The CPU does not use them. `lib.rs` also does not export memory yet.

- `Memory` — fixed-size zeroed store, read returns `None` off the end, write returns `false`
- `Register` — read / write / clear
- `Accumulator` — a named register that starts at 0
- `RegisterFile` — four registers, index 0–3, rejects anything past that

Next work is to hang these off `Machine` so execute talks to an accumulator and a register file, and so data memory is separate from the program bytes. Load/store opcodes would follow from that.


## Not started, or just empty files

- `src/alu/` is still empty stubs
- checksums
- a binary program format (right now a program is just a `Vec<u8>`)
- more than one general-purpose register in the live CPU
- anything past JUMP_IF_ZERO for control flow

The list at the top of this file is the destination, not a claim that all of it already runs.