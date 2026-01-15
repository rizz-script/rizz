# Agent Instructions (Cursor / agentic IDEs)

This repo contains the **RizzScript** interpreter (Rust).

## Quick commands

- Build: `cargo build`
- Run a script: `cargo run -- run examples/hello.rizz`
- Format scripts: `cargo run -- format examples`
- Init starter: `cargo run -- init`

## Repo conventions

- Prefer **small commits** with clear messages.
- Keep changes compatible with **Rust 1.82 / Cargo 1.82**.
- Avoid adding dependencies unless needed; pin versions if MSRV bumps occur.

## Implementation notes

- The language is interpreted; parsing lives in `src/lexer.rs` and `src/parser.rs`.
- Runtime/builtins live in `src/runtime.rs`.
- `Listen(port, handler)` supports both raw TCP and basic HTTP detection.

