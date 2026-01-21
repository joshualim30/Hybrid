# Hybrid Architecture & Project Status

## 1. Project Overview
Hybrid is a polyglot programming language that embeds foreign code blocks (Python, Rust) directly into its native execution flow. It is built in Rust using a tree-walk interpreter architecture.

### Core Components
- **Lexer/Parser**: Recursive descent parser generating an AST. Tracks line/column for LSP.
- **Evaluator**: Tree-walk interpreter. Manages scope, variables, and control flow.
- **Polyglot Runtime**: A sub-system that manages external language processes.
- **LSP Server**: Implements the Language Server Protocol for IDE features.

## 2. Technical Deep Dive

### The Polyglot Runtime
Instead of transpiling the entire file to a target language, Hybrid executes "Host" code natively and delegates "Foreign" blocks to their respective runtimes on-the-fly.

#### Data Bridge
Communication happens via Standard Streams (STDIN/STDOUT) using JSON serialization.
- **Hybrid -> Foreign**: Arguments are serialized to JSON.
- **Foreign -> Hybrid**: Results are printed to STDOUT as JSON, captured, and deserialized.

#### Python Implementation
- **Execution**: Stateless subprocess spawning (`python3 -c ...`).
- **Wrapper**: A shim script is generated around the user's code to map JSON inputs to local variables and capture return values.

#### Rust Implementation
- **JIT-like Execution**: The Rust block is wrapped in a `main` function, compiled to a temporary binary in `std::env::temp_dir()`, executed, and then immediately deleted.
- **Performance**: High latency per-call due to compilation overhead (room for optimization: persistent daemon or incremental compilation).

### Type System
Hybrid uses a strong, dynamic type system during evaluation.
- **Primitives**: `int` (i64), `float` (f64), `bool`, `string`.
- **Collections**: `array` and `map` are supported in native code.
- **Immutability**: `const` vs `var` is enforced at runtime by the Evaluator.

**Limitation**: Currently, `array` and `map` types are **not** marshalled to foreign runtimes. Only primitives can cross the boundary.

## 3. Current Status Matrix

| Feature | Status | Notes |
| :--- | :--- | :--- |
| **Control Flow** | ✅ Working | `if`, `else`, `while`, `block` |
| **Variables** | ✅ Working | `var`, `const`, Scoped |
| **Collections** | ⚠️ Partial | Works natively, but cannot pass to `#python`/`#rust` |
| **Polyglot: Python** | ✅ Working | JSON IPC working well for primitives |
| **Polyglot: Rust** | ✅ Working | Compilation works, but slow (hot-path issue) |
| **LSP** | ✅ Working | Diagnostics & Basic Autocomplete |
| **VS Code Ext** | ✅ Working | Syntax Highlighting + Language Server Client |
| **Error Handling** | ⚠️ Basic | Simple string errors. No stack traces across boundaries |
| **Std Lib** | ❌ Missing | Only `speak()` exists. No File I/O, Math, etc. |

## 4. Future Roadmap (To Be Implemented)
1.  **Orchestrator**: Complex data pipelines (e.g., Python output -> Rust input).
2.  **Collection Marshalling**: Support passing Lists/Dicts to Python and Vectors/HashMaps to Rust.
3.  **Language Daemons**: Keep Python/Rust processes alive to avoid startup overhead.
4.  **Debugger**: Debug Adapter Protocol (DAP) implementation.
