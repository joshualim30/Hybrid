# Hybrid Programming Language

<div align="center">
  <img src="extension/images/hybrid_logo.png" alt="Hybrid Logo" width="128" />
</div>

**Hybrid** is a modern, polyglot programming language designed to bridge the gap between high-performance systems programming and dynamic scripting. It features a robust type system, explicit control flow, and a unique runtime that executes Python and Rust code blocks natively.

## Key Features

- **Polyglot Runtime**: Embed `#python` and `#rust` blocks directly in your code.
- **Strong Typing**: `int`, `float`, `string`, `bool`, `array[T]`, `map[K, V]`.
- **Modern Syntax**:
  - `var` (mutable) and `const` (immutable) declarations.
  - `block` keyword for function definitions.
  - Explicit `return` statements.
- **Developer Tools**:
  - Built-in Language Server (LSP) with diagnostics and autocomplete.
  - VS Code Extension with syntax highlighting.
  - CLI with `init`, `doctor`, and `run` commands.

## Installation

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/joshualim30/hybrid.git
    cd hybrid/language
    ```

2.  **Build and Install CLI:**
    ```bash
    cargo install --path .
    ```

3.  **Install VS Code Extension:**
    - Navigate to `extension/`.
    - Run `npm install && npm run compile`.
    - Package: `npx vsce package`.
    - Install the resulting `.vsix` file in VS Code.

## Usage

### Writing Hybrid Code
Create a file named `hello.hyb`:

```hybrid
// Native Hybrid code
string const greeting = "Hello";
int var count = 0;

// Embed Python for data processing
#python
string block py_process(string name) {
    return f"Processed {name} in Python"
}

// Embed Rust for performance
#rust
int block rs_square(int n) {
    let x: i64 = n.parse().unwrap();
    x * x
}

speak(greeting, "Hybrid!");
speak(py_process("Data"));
speak(rs_square(12));
```

### Running Code
```bash
hybrid run hello.hyb
```

### CLI Commands
- `hybrid init <name>`: Create a new project.
- `hybrid doctor`: Check your environment (Rust/Python availability).
- `hybrid run <file>`: Execute a Hybrid file.
- `hybrid transpile <file> <target>`: Transpile to Python or Rust.

## License

MIT
