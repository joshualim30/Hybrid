# Hybrid Language Support for VS Code

🔥 **Version 0.1.0** - VS Code extension for the Hybrid programming language.

## 🚀 Features

### **Basic Language Support**
- **Syntax Highlighting**: Full highlighting for .hyb files
- **Basic IntelliSense**: Autocompletion for keywords and functions
- **Hover Help**: Documentation for basic language features

### **CLI Integration**
- **Run Hybrid Files**: Right-click → "Run Hybrid File" or `Ctrl+F5`
- **Interactive REPL**: `Ctrl+Alt+R` to open Hybrid REPL

## 📁 Supported File Extensions

- `.hyb` - Hybrid language files

## ⌨️ Keyboard Shortcuts

| Command | Windows/Linux | Mac | Description |
|---------|--------------|-----|-------------|
| Run File | `Ctrl+F5` | `Cmd+F5` | Execute current Hybrid file |
| Open REPL | `Ctrl+Alt+R` | `Cmd+Alt+R` | Start interactive shell |

## 🛠️ Installation

### Manual Installation
1. Copy the extension folder to your VS Code extensions directory:
   - **Windows**: `%USERPROFILE%\.vscode\extensions\`
   - **macOS**: `~/.vscode/extensions/`
   - **Linux**: `~/.vscode/extensions/`
2. Restart VS Code

## 🔧 Requirements

- **VS Code**: 1.75.0 or higher
- **Hybrid Interpreter**: Built from the `language` directory

## 📝 Code Examples

### Basic Syntax
```hybrid
// Variables
var x = 10;
const message = "Hello, Hybrid!";

// Functions
block add(a, b) {
    return a + b;
}

// Function calls
var result = add(5, 3);
speak("Result:", result);
```

## 🎯 Available Commands

Access via Command Palette (`Ctrl+Shift+P`):

- `Hybrid: Run Hybrid File` - Execute current file
- `Hybrid: Open Hybrid REPL` - Start interactive session

## 🐛 Troubleshooting

### "hybrid command not found"
Make sure the Hybrid interpreter is built:
```bash
cd language
cargo build
```

### Extension not activating
1. Check file extension is `.hyb`
2. Restart VS Code
3. Check Extensions panel for any errors

## 🔄 Development

To modify this extension:

1. **Install dependencies**: `npm install`
2. **Compile**: `npm run compile`  
3. **Watch for changes**: `npm run watch`

## 📄 License

MIT License

---

**Created by Joshua Lim** | Hybrid Language Extension v0.1.0

