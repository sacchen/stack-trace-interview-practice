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
- `mock_interview_packet.md`: an optional 3-part mock interview packet; Part 1 is the main exercise
- `src/part2_call_tree.rs`: optional Rust template for the Part 2 follow-up
- `src/part3_stats.rs`: optional Rust template for the Part 3 follow-up
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

If you want to run it as a structured mock interview with a friend, use
`mock_interview_packet.md`, but treat Part 1 as the default stopping point.
Only continue to Parts 2 and 3 if they still feel educational after finishing
or discussing Part 1.

If you decide to continue later, there are separate optional follow-up stubs in:

- `src/part2_call_tree.rs`
- `src/part3_stats.rs`

You can run those separately with:

```bash
cargo run --bin part2_call_tree
cargo run --bin part3_stats
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
- The main exercise is Part 1: converting sampled stacks into a valid trace.
- This problem setup was adapted for personal interview practice.
