# Hybrid Programming Language

<div align="center">
  <img src="extension/images/hybrid_logo.png" alt="Hybrid Logo" width="160" />
  <h2>Flexible Runtimes for Flexible Environments</h2>

  [![Build Status](https://github.com/joshualim30/hybrid/actions/workflows/build.yml/badge.svg)](https://github.com/joshualim30/hybrid/actions)
  [![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
  [![Version](https://img.shields.io/badge/version-0.1.0-orange)](https://github.com/joshualim30/hybrid/releases)
  [![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](CONTRIBUTING.md)
  [![VS Code](https://img.shields.io/badge/VS%20Code-Extension-007ACC?logo=visualstudiocode)](https://marketplace.visualstudio.com)
</div>

---

### Overview

**Hybrid** is a high-performance polyglot programming language engineered to solve the friction between high-level scripting and systems-level programming. It allows developers to embed **Python** and **Rust** blocks directly within a unified Hybrid source file, leveraging the strengths of each language without the burden of complex build systems or FFI boilerplate.

> "Write the orchestration in Hybrid, the data logic in Python, and the bottlenecks in Rust."

---

### Key Features

*   **Native Polyglot Execution**: Define `#python` and `#rust` blocks that execute as first-class citizens within the Hybrid runtime.
*   **Strongly Typed Architecture**: A robust type system including `int`, `float`, `string`, `bool`, `array`, and `map`.
*   **Automated Interoperability**: Transparent JSON-based data marshalling between language boundaries.
*   **Built-in LSP Support**: Full Language Server Protocol implementation providing real-time diagnostics and autocompletion.
*   **Developer-First Tooling**: Zero-configuration environment—simply run `hybrid run`.

---

### Getting Started

#### Prerequisites

*   **Rust**: `cargo` 1.70+
*   **Python**: `python3.10+`
*   **Node.js**: `npm` (for IDE extensions)

#### Installation

1.  **Build the Core CLI**:
    ```bash
    git clone https://github.com/joshualim30/hybrid.git
    cd hybrid/language
    cargo install --path .
    ```

2.  **Environment Check**:
    ```bash
    hybrid doctor
    ```

3.  **IDE Extension**:
    *   Navigate to the `extension/` directory.
    *   Initialize dependencies and compile: `npm install && npm run compile`.
    *   Install the generated `.vsix` file to VS Code.

---

### Quick Start Example

Create `main.hyb`:

```javascript
// Native Hybrid declaration
string const project = "Hybrid Engine";

// Compute heavy tasks in Rust
#rust
int block fibonacci(int n) {
    if n <= 1 { return n; }
    let (mut a, mut b) = (0, 1);
    for _ in 0..n {
        let temp = a;
        a = b;
        b = temp + b;
    }
    b
}

// Data processing in Python
#python
string block welcome_msg(string title) {
    import datetime
    ts = datetime.datetime.now().strftime("%H:%M:%S")
    return f"[{ts}] Initializing {title}..."
}

// Orchestration in Hybrid
speak(welcome_msg(project));
int var result = fibonacci(12);
speak("Computation Result:", result);
```

Run the program:
```bash
hybrid run main.hyb
```

---

### Architecture

Hybrid operates as a tree-walk interpreter written in Rust. It utilizes a sophisticated **Runtime Manager** to handle bridge communication:

1.  **AST Transformation**: Hybrid parses source code into an Abstract Syntax Tree, isolating foreign blocks.
2.  **Stateless Bridging**: Arguments are serialized to JSON and passed via standard streams to specialized language shims.
3.  **Dynamic Execution**: Python blocks use transient subprocesses, while Rust blocks are compiled on-demand in a temporary workspace for maximum performance.

Detailed documentation: [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md)

---

### Roadmap

*   [x] Native Control Flow & Dynamic Scoping
*   [x] Initial Polyglot Bridges (Python & Rust)
*   [x] VS Code Extension & Language Server (LSP)
*   [ ] **Standard Library**: File IO, Networking, and Cryptography
*   [ ] **Process Daemons**: Persistent runtimes for low-latency foreign calls
*   [ ] **Hybrid Orchestrator**: Direct piping between foreign runtimes

---

### Contributing

We are an open-source project and welcome all types of contributions! Please read our [Contributing Guide](CONTRIBUTING.md) to get started.

---

### Contact & Support

*   **Official Website**: [devhybrid.org](https://devhybrid.org)
*   **Email**: [contact@devhybrid.org](mailto:contact@devhybrid.org)

---
<div align="center">
  <sub>Built with Excellence by the Hybrid Team</sub>
</div>
