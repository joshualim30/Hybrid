# Installing the Hybrid Language Extension v0.1.0

## Method 1: Manual Installation (Recommended)

1. **Copy the extension to VS Code extensions folder:**

   **macOS:**
   ```bash
   cp -r extension ~/.vscode/extensions/hybrid-language-0.1.0
   ```

   **Windows:**
   ```bash
   xcopy new\extension "%USERPROFILE%\.vscode\extensions\hybrid-language-0.1.0" /E /I
   ```

   **Linux:**
   ```bash
   cp -r extension ~/.vscode/extensions/hybrid-language-0.1.0
   ```

2. **Reload VS Code:**
   - Press `Ctrl+Shift+P` (or `Cmd+Shift+P` on Mac)
   - Type "Developer: Reload Window"
   - Press Enter

3. **Test the extension:**
   - Open any `.hyb` file (like `tests/basic.hyb`)
   - You should see syntax highlighting
   - Press `Ctrl+F5` (or `Cmd+F5`) to run the file

## Verify Installation

1. Open VS Code
2. Open the file `tests/basic.hyb`
3. You should see:
   - Syntax highlighting (keywords in blue, functions in yellow, etc.)
   - Auto-completion when typing keywords like `var`, `const`, `block`
   - Right-click context menu with "Run Hybrid File"
   - Hover help on keywords

## Troubleshooting

- **No syntax highlighting:** Make sure the file has `.hyb` extension
- **Can't run files:** Ensure your Hybrid interpreter is compiled (`cd language && cargo build`)
- **Extension not loading:** Check VS Code Developer Console (`Help > Toggle Developer Tools`)

## Testing the Extension

Create a test file:
```hybrid
// test.hyb
var x = 42;
const message = "Hello, World!";

block greet(name) {
    return "Hello, " + name + "!";
}

speak(greet("Hybrid"));
speak("x =", x);
```

Save it as `test.hyb` and press `Ctrl+F5` to run it.

