# Rust Learning Curriculum for Python Developers

This repo hosts a hands-on curriculum tailored for experienced Python developers who want to pick up Rust by building and iterating on real code. The roadmap is organized as a sequence of small, focused modules that compound into two guided projects: a command-line todo manager and an async file sync service. Each module includes clear goals, exercises, stretch tasks, and suggested references to deepen your understanding.

---

## Getting Set Up

- Ensure the Rust toolchain is installed: `source "$HOME/.cargo/env" && rustc --version`
- Create a sandbox area for experimentation: `cargo new lab && cd lab`
- Install VS Code extensions or use `rust-analyzer` in your editor of choice
- Recommended reading reference (skim only as needed): [Rust Book](https://doc.rust-lang.org/book/), [Rust By Example](https://doc.rust-lang.org/stable/rust-by-example/)

> 💡 Treat this curriculum as a workbook. Commit your progress regularly and keep notes on what confuses you.

---

## Progress Tracker

Use this checklist to track your progress. Each module builds on the last, so follow them in order.

- [ ] Module 0 · Toolchain, Cargo, fmt, and Clippy
- [ ] Module 1 · Variables, immutability, and type inference
- [ ] Module 2 · Control flow and pattern matching
- [ ] Module 3 · Functions, ownership, and borrowing
- [ ] Module 4 · Structs, enums, and pattern design
- [ ] Module 5 · Collections, iterators, and generics
- [ ] Module 6 · Error handling and testing
- [ ] Module 7 · Modularization and crates
- [ ] Module 8 · Lifetimes and references in depth
- [ ] Module 9 · Concurrency primitives
- [ ] Module 10 · Async foundations and networking
- [ ] Capstone A · CLI todo manager
- [ ] Capstone B · Async file sync mini-service

---

## Module 0 · Toolchain, Cargo, fmt, and Clippy

**Objectives**
- Understand the role of `rustup`, the default toolchain, and how to update it
- Create new projects with `cargo new`, run, build, and test them
- Format code with `cargo fmt` and lint with `cargo clippy`

**Exercises**
1. Run `rustup show` and note the active toolchain.
2. `cargo new hello_cargo` and explore the generated layout.
3. Modify `main.rs` to print your name, fmt it, then run clippy.
4. Add a simple test that asserts `2 + 2 == 4` and run `cargo test`.

**Stretch**
- Install an alternative toolchain (`rustup toolchain install nightly`) and try `cargo +nightly fmt`.

---

## Module 1 · Variables, Immutability, and Type Inference

**Objectives**
- Declare variables with `let`, shadow them, and use mutability when required
- Explore primitive types (integers, floats, bools, chars)
- Learn basic string handling with `String` & `&str`

**Exercises**
1. Build a program that parses command-line args into integers and prints their sum.
2. Practice shadowing by transforming a variable from a string slice to an owned `String`.
3. Write a function that toggles a boolean flag using `mut` and returns it.

**Stretch**
- Experiment with numeric literals (`0b`, `0x`, `_` separators) and print them.

---

## Module 2 · Control Flow and Pattern Matching

**Objectives**
- Use `if`, `else`, and `match` to branch logic
- Iterate with `loop`, `while`, and `for`
- Destructure tuples and enums inside matches

**Exercises**
1. Build a simple fizz-buzz implementation with pattern matching.
2. Parse CLI input into an enum representing commands (e.g., `Add`, `List`, `Quit`).
3. Use `while let` to pop elements from a `Vec` until empty.

**Stretch**
- Implement a mini REPL that supports `add` and `mul` operations using `match`.

---

## Module 3 · Functions, Ownership, and Borrowing

**Objectives**
- Define functions that take ownership vs borrowing parameters
- Understand move semantics and the `Copy` trait
- Work with borrowing rules and mutable references

**Exercises**
1. Write `fn describe(input: &String)` and observe why it expects a reference.
2. Implement a `fn take_and_return(v: Vec<i32>) -> Vec<i32>` and explain the ownership transfer.
3. Build a function that takes `&mut Vec<String>` and appends items safely.

**Stretch**
- Use `String::from_utf8` to convert a byte vector and handle the `Result`.

---

## Module 4 · Structs, Enums, and Pattern Design

**Objectives**
- Define tuple, unit, and named structs
- Create enums with data-bearing variants and `impl` blocks
- Leverage `match` and `if let` for ergonomic control flow

**Exercises**
1. Model a `TaskStatus` enum (`Todo`, `Doing`, `Done { finished_at: DateTime<Utc> }`).
2. Implement `impl Task` with builder methods returning `Self` for chaining.
3. Pattern match on `Option<Task>` to print status updates.

**Stretch**
- Use `derive` macros (`Debug`, `Clone`, `PartialEq`) and customize `fmt::Display`.

---

## Module 5 · Collections, Iterators, and Generics

**Objectives**
- Work with `Vec`, `HashMap`, and `HashSet`
- Chain iterator adapters (`map`, `filter`, `collect`)
- Introduce generics and trait bounds for reusable functions

**Exercises**
1. Build a frequency counter for words in a string (store counts in `HashMap`).
2. Implement a generic `fn pick_first<T: Clone>(items: &[T]) -> Option<T>`.
3. Refactor fizz-buzz using iterators and `collect::<Vec<_>>()` to a single string.

**Stretch**
- Implement your own iterator struct with `impl Iterator for ...`.

---

## Module 6 · Error Handling and Testing

**Objectives**
- Understand `Result`, the `?` operator, and custom error enums
- Write unit and integration tests with `#[cfg(test)]`
- Implement `From`/`Into` for error conversions

**Exercises**
1. Build a CSV parser that returns `Result<Vec<Record>, ParseError>`.
2. Add unit tests for success and failure cases.
3. Use `thiserror` crate (add to `Cargo.toml`) to derive error implementations.

**Stretch**
- Add property-based tests with `proptest` or `quickcheck`.

---

## Module 7 · Modularization and Crates

**Objectives**
- Organize code into modules (`mod`, `pub`, `use`)
- Split logic across files and directories
- Understand crate roots, workspace layout, and dependency management

**Exercises**
1. Refactor a previous module's code into `src/lib.rs` + `src/bin/main.rs`.
2. Create a workspace with two crates sharing a common library.
3. Publish a dummy crate to a local git repo and add it as a dependency.

**Stretch**
- Explore feature flags in `Cargo.toml` and implement a `serde`-optional serialization path.

---

## Module 8 · Lifetimes and References in Depth

**Objectives**
- Recognize when lifetimes are inferred vs explicit
- Implement functions that return references tied to inputs
- Use struct fields containing references with lifetime annotations

**Exercises**
1. Implement `fn longest<'a>(a: &'a str, b: &'a str) -> &'a str` and reason about `'a`.
2. Build a `struct Borrowed<'a> { data: &'a str }` and instantiate it correctly.
3. Convert a function that clones aggressively into one that borrows efficiently.

**Stretch**
- Document lifetime elision rules by rewriting function signatures with explicit lifetimes.

---

## Module 9 · Concurrency Primitives

**Objectives**
- Use threads with `std::thread` and message passing (`std::sync::mpsc`)
- Share state safely via `Arc<Mutex<T>>`
- Understand data races and ownership in threaded contexts

**Exercises**
1. Spawn multiple threads to compute partial sums and join them.
2. Implement a producer/consumer channel with `mpsc::channel`.
3. Build a shared counter protected by `Arc<Mutex<i32>>`.

**Stretch**
- Experiment with `Rayon` for data parallel iterators.

---

## Module 10 · Async Foundations and Networking

**Objectives**
- Understand Rust's async/await model and executors
- Use `tokio` to write async code and manage tasks
- Perform simple HTTP requests with `reqwest`

**Exercises**
1. Add `tokio` and `reqwest` to a new crate, then fetch JSON from an API.
2. Implement an async file watcher that logs changes (`notify` + async channels).
3. Introduce structured logging with `tracing`.

**Stretch**
- Benchmark async vs sync versions of the same HTTP fetcher using `cargo bench`.

---

## Capstone A · CLI Todo Manager

Bring together Modules 0–7 by building a small but robust CLI application.

**Milestones**
1. Design a data model for tasks (`struct Task`, `enum Priority`).
2. Implement `add`, `list`, `complete`, and `delete` commands using `clap` or `argparse` crates.
3. Persist tasks to disk (start with JSON, stretch to `ron` or `toml`).
4. Add unit tests for business logic and integration tests that spawn the binary.
5. Package the tool with `cargo install --path .` and try it system-wide.

**Extensions**
- Add due dates and human-friendly parsing with `chrono`
- Implement search/filter functionality and output formatting
- Export and import tasks between JSON and CSV

---

## Capstone B · Async File Sync Mini-Service

Combine async, networking, and ownership lessons into a non-trivial project.

**Milestones**
1. Create a binary crate `syncer` with async main using `tokio`.
2. Watch a directory for changes (`notify` crate) and broadcast events over a WebSocket (`tokio-tungstenite`).
3. Implement a client that listens for events and makes HTTP PUT requests to upload changed files.
4. Add retry logic with exponential backoff and structured errors via `thiserror`.
5. Write integration tests using `tokio::test` and a fake HTTP server (`wiremock`).

**Extensions**
- Explore streaming large files with `tokio::fs` and chunked uploads.
- Add authentication and config loading via environment variables or `.env` files.

---

## Suggested Weekly Structure

| Day | Focus | Deliverable |
| --- | ----- | ----------- |
| Mon | Read module overview + official docs | Annotated notes in repo |
| Tue | Implement exercises | Passing `cargo test` |
| Wed | Stretch goals or refactor | Clean git diff |
| Thu | Pair with a peer or explain concepts aloud | Recorded walkthrough or README notes |
| Fri | Review + summarize | Journal entry highlighting wins + blockers |

---

## Habit Checklist

- Run `cargo fmt` and `cargo clippy` before every commit
- Write tests for new functionality
- Keep `README.md` updated with insights and questions
- Reflect weekly: what from Python knowledge helps? what feels new?

---

## Next Steps

1. Complete Module 0 using this repo (`cargo new lab` inside `projects/` folder).
2. Commit your progress and notes.
3. When you hit friction, document it in a `diary/` directory for future reference.

Happy hacking! Embrace the borrow checker—it becomes a superpower with practice.
