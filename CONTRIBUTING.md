# Contributing to Hybrid Programming Language

Thank you for your interest in contributing to the Hybrid programming language! This document provides guidelines and information for contributors.

## 🚀 Getting Started

### Prerequisites
- [Rust](https://rustup.rs/) (latest stable version)
- [Git](https://git-scm.com/)
- [VS Code](https://code.visualstudio.com/) (recommended for development)

### Development Setup

1. **Fork and clone the repository:**
   ```bash
   git clone https://github.com/yourusername/hybrid.git
   cd hybrid
   ```

2. **Build the project:**
   ```bash
   cd language
   cargo build
   ```

3. **Run tests:**
   ```bash
   cargo test
   ```

4. **Test with example files:**
   ```bash
   ./target/debug/hybrid ../tests/basic.hyb
   ```

## 🛠️ Development Workflow

### Making Changes

1. **Create a feature branch:**
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes** following the coding standards below

3. **Test your changes:**
   ```bash
   cargo test
   cargo build
   ./target/debug/hybrid ../tests/basic.hyb
   ```

4. **Commit your changes:**
   ```bash
   git add .
   git commit -m "Add: brief description of your changes"
   ```

5. **Push to your fork:**
   ```bash
   git push origin feature/your-feature-name
   ```

6. **Open a Pull Request** on GitHub

### Commit Message Guidelines

Use clear, descriptive commit messages:
- `Add: new feature or functionality`
- `Fix: bug fixes`
- `Update: improvements to existing features`
- `Docs: documentation changes`
- `Test: adding or updating tests`

Examples:
```
Add: support for while loops in parser
Fix: division by zero error handling
Update: improve error messages in lexer
Docs: add examples for function syntax
```

## 📝 Coding Standards

### Rust Code Style
- Follow standard Rust formatting (`cargo fmt`)
- Use `cargo clippy` to catch common issues
- Add documentation comments for public functions
- Write unit tests for new functionality

### File Organization
```
language/src/
├── main.rs      # Main interpreter entry point
├── lexer.rs     # Tokenization logic
├── parser.rs    # Parsing logic  
├── ast.rs       # Abstract Syntax Tree definitions
└── evaluator.rs # Code execution logic
```

### Code Examples
```rust
// Good: Clear function with documentation
/// Tokenizes the input string into a vector of tokens
pub fn tokenize(&mut self) -> Vec<Token> {
    let mut tokens = Vec::new();
    // Implementation...
    tokens
}

// Good: Descriptive variable names
let function_name = self.read_identifier();
let parameter_count = parameters.len();

// Good: Error handling
match self.parse_expression() {
    Ok(expr) => expr,
    Err(e) => return Err(format!("Parse error: {}", e)),
}
```

## 🧪 Testing

### Running Tests
```bash
# Run all tests
cargo test

# Run specific test
cargo test test_lexer

# Run with output
cargo test -- --nocapture
```

### Writing Tests
Add tests for new functionality:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variable_declaration() {
        let mut lexer = Lexer::new("var x = 42;");
        let tokens = lexer.tokenize();
        
        assert_eq!(tokens[0], Token::Var);
        assert_eq!(tokens[1], Token::Identifier("x".to_string()));
        // ... more assertions
    }
}
```

### Manual Testing
Test your changes with example files:
```bash
# Create a test file
echo 'var x = 10; speak("x =", x);' > test.hyb

# Run it
./target/debug/hybrid test.hyb
```

## 📚 Areas for Contribution

### Language Features
- **Control Flow**: if/else statements, loops (for, while)
- **Data Structures**: arrays, objects/maps
- **Standard Library**: math functions, string operations
- **Error Handling**: try/catch mechanisms
- **Type System**: optional type annotations

### Tooling
- **VS Code Extension**: improved IntelliSense, debugging support
- **Language Server**: LSP implementation for better IDE support
- **Package Manager**: dependency management system
- **Documentation**: language reference, tutorials

### Performance
- **Optimization**: faster parsing and execution
- **Memory Management**: improved garbage collection
- **Benchmarking**: performance testing suite

## 🐛 Reporting Issues

### Bug Reports
When reporting bugs, please include:
- **Description**: What happened vs. what you expected
- **Code Sample**: Minimal example that reproduces the issue
- **Environment**: OS, Rust version, etc.
- **Error Output**: Full error messages or stack traces

### Feature Requests
For new features, please describe:
- **Use Case**: Why is this feature needed?
- **Proposed Syntax**: How should it work?
- **Examples**: Code samples showing the feature in use
- **Alternatives**: Other ways to achieve the same goal

## 📋 Pull Request Guidelines

### Before Submitting
- [ ] Code builds without warnings (`cargo build`)
- [ ] All tests pass (`cargo test`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] No clippy warnings (`cargo clippy`)
- [ ] Documentation is updated if needed
- [ ] Examples work with your changes

### PR Description
Include in your PR description:
- **Summary**: What does this PR do?
- **Changes**: List of specific changes made
- **Testing**: How did you test the changes?
- **Breaking Changes**: Any backwards compatibility issues?

## 🤝 Code Review Process

1. **Automated Checks**: CI will run tests and linting
2. **Maintainer Review**: A project maintainer will review your code
3. **Feedback**: Address any requested changes
4. **Approval**: Once approved, your PR will be merged

## 📞 Getting Help

- **Questions**: Open a [Discussion](https://github.com/joshualim30/hybrid/discussions)
- **Issues**: Check existing [Issues](https://github.com/joshualim30/hybrid/issues)
- **Chat**: Join our community discussions

## 🏆 Recognition

Contributors will be:
- Listed in the project's contributors
- Mentioned in release notes for significant contributions
- Invited to join the maintainer team for ongoing contributors

Thank you for contributing to Hybrid! 🎉
