# CLI Interface

The Hybrid CLI is your Swiss Army knife for polyglot development.

## Commands

- `hybrid run <file>`: Execute a hybrid file.
- `hybrid build <file>`: Compile to a standalone binary.
- `hybrid doctor`: Check health of foreign runtimes.
- `hybrid add <runtime>`: Install a new runtime bridge.

## Flags

- `--release`: Optimize for speed (longer compile time).
- `--target`: Specify build target (e.g. `wasm32-unknown-unknown`).
