# Hybrid Programming Language

A clean, minimalist programming language built with Rust, designed for ease of use.

## 🚀 Features

- **Clean Syntax**: Readable code with minimal boilerplate
- **Fast Execution**: Built with Rust for performance and safety
- **Interactive REPL**: Test code snippets instantly
- **VS Code Support**: Full IDE integration with syntax highlighting and IntelliSense
- **Cross-Platform**: Works on Windows, macOS, and Linux

## 📝 Quick Example

```hybrid
// Variables and constants
var x = 10;
const message = "Hello, Hybrid!";

// Functions (called "blocks")
block add(a, b) {
    return a + b;
}

// Function calls and output
var result = add(x, 20);
speak("Result:", result); // Output: Result: 30
```

## 🛠️ Installation

### Prerequisites
- [Rust](https://rustup.rs/) (latest stable version)
- [VS Code](https://code.visualstudio.com/) (optional, for IDE support)

### Build from Source

1. **Clone the repository:**
   ```bash
   git clone https://github.com/yourusername/hybrid.git
   cd hybrid
   ```

2. **Build the interpreter:**
   ```bash
   cd language
   cargo build --release
   ```

3. **Install globally (optional):**
   ```bash
   cargo install --path .
   ```

### VS Code Extension

1. **Install the extension:**
   ```bash
   cp -r extension ~/.vscode/extensions/hybrid-language-0.1.0
   ```

2. **Reload VS Code** and open any `.hyb` file to see syntax highlighting.

## 🎯 Usage

### Running Files
```bash
# Run a Hybrid file
./target/release/hybrid example.hyb

# Or if installed globally
hybrid example.hyb
```

### Interactive REPL
```bash
# Start the REPL
./target/release/hybrid

# Or if installed globally
hybrid
```

### VS Code Integration
- **Run File**: `Ctrl+F5` (or `Cmd+F5` on Mac)
- **Open REPL**: `Ctrl+Alt+R` (or `Cmd+Alt+R` on Mac)
- **Right-click** any `.hyb` file → "Run Hybrid File"

## 📚 Language Reference

### Variables
```hybrid
var x = 42;           // Mutable variable
const pi = 3.14159;   // Immutable constant
```

### Data Types
```hybrid
var number = 42;           // Numbers (integers and floats)
var text = "Hello!";       // Strings
var flag = true;           // Booleans (true/false)
```

### Functions
```hybrid
// Function definition
block greet(name) {
    return "Hello, " + name + "!";
}

// Function call
var greeting = greet("World");
speak(greeting);
```

### Arithmetic
```hybrid
var a = 10;
var b = 3;

speak(a + b);  // Addition: 13
speak(a - b);  // Subtraction: 7
speak(a * b);  // Multiplication: 30
speak(a / b);  // Division: 3.33...
```

### Built-in Functions
```hybrid
speak("Hello");           // Print to console
speak("Value:", 42);      // Print multiple values
```

## 📁 Project Structure

```
hybrid/
├── language/            # Rust interpreter source
│   ├── src/
│   │   ├── main.rs      # Main interpreter
│   │   ├── lexer.rs     # Tokenizer
│   │   ├── parser.rs    # Parser
│   │   ├── ast.rs       # Abstract Syntax Tree
│   │   └── evaluator.rs # Code execution
│   └── Cargo.toml
├── extension/           # VS Code extension
│   ├── src/
│   ├── syntaxes/
│   └── package.json
├── tests/               # Example files
├── resources/           # Project assets
└── README.md
```

## 🧪 Examples

Check out the `tests/` directory for example programs:

- `tests/basic.hyb` - Basic language features
- More examples coming soon!

## 🤝 Contributing

We welcome contributions! Here's how to get started:

1. **Fork the repository**
2. **Create a feature branch:** `git checkout -b feature/amazing-feature`
3. **Make your changes** and add tests
4. **Commit your changes:** `git commit -m 'Add amazing feature'`
5. **Push to the branch:** `git push origin feature/amazing-feature`
6. **Open a Pull Request**

### Development Setup

```bash
# Clone your fork
git clone https://github.com/yourusername/hybrid.git
cd hybrid

# Build and test
cd language
cargo build
cargo test

# Test with example files
./target/debug/hybrid ../tests/basic.hyb
```

## 🐛 Issues and Support

- **Bug Reports**: [Open an issue](https://github.com/joshualim30/hybrid/issues)
- **Feature Requests**: [Start a discussion](https://github.com/joshualim30/hybrid/discussions)
- **Questions**: Check existing issues or start a new discussion

## 🗺️ Roadmap

- [ ] Control flow (if/else, loops)
- [ ] Arrays and data structures
- [ ] Standard library functions
- [ ] Package system
- [ ] Error handling improvements
- [ ] Performance optimizations

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- VS Code extension inspired by the official language extension examples
- Thanks to all contributors and users!

---

**Created by Joshua Lim** | [GitHub](https://github.com/joshualim30) | [Website](https://joshualim.me)
