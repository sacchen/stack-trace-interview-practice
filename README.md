# Stack Trace Interview Practice

Small practice repo for a mock interview problem about converting sampled call
stacks into trace events.

This repository is intentionally minimal:

- a Rust version for the mock interview
- a C++ version for comparison
- no solution checked in
- no heavy scaffolding or test framework

## What's In The Repo

- `src/main.rs`: the main Rust exercise file
- `rust_notes.md`: small Rust syntax reminders that are fair to reference in a mock interview
- `Cargo.toml`: Rust project configuration
- `src/main.cpp`: the same prompt in C++
- `run`: quick C++ build-and-run script

## Recommended Setup (Rust)

From the project root:

```bash
cargo run
```

That will:

- compile `src/main.rs`
- keep debug info in the default dev build
- run the program

If you only want to typecheck without running:

```bash
cargo check
```

## Recommended Mock Interview Workflow

Use two terminal panes:

1. `nvim -u NONE -N src/main.rs`
2. `cargo run`

If you want a small syntax reference open beside the editor, use:

```bash
nvim -u NONE -N rust_notes.md
```

## Optional C++ Workflow

If you want to practice the C++ version instead:

```bash
./run
```

That script compiles `src/main.cpp` with warnings, debug info, and sanitizers.

## Notes

- `convert_to_trace` in Rust and `convertToTrace` in C++ are intentionally stubbed.
- The point of this repo is to practice reasoning, not to store the answer.
- This problem setup was adapted for personal interview practice.
