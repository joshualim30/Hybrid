# Changelog

All notable changes to this project will be documented in this file.

## [0.1.0] - 2026-01-21

### Added
- **Polyglot Runtime**: Support for `#python` and `#rust` code blocks.
- **Language Server**: Initial LSP implementation with `tower-lsp`.
  - Real-time syntax validation.
  - Autocomplete for keywords and types.
- **CLI improvements**:
  - `hybrid init` for scaffolding.
  - `hybrid doctor` with auto-fix suggestions.
  - `hybrid --version` and `--help` flags.
- **Type System**:
  - Added `array[T]` and `map[K, V]` generic types.
  - Enforced `const` immutability.
- **VS Code Extension**:
  - Semantic syntax highlighting.
  - Integration with `hybrid-lsp`.

### Changed
- Refactored `evaluator` to support runtime execution of foreign blocks.
- Migrated CLI to `clap` v4.
- Updated project structure for better modularity (`lexer`, `parser`, `ast`, `evaluator`, `runtime`).

### Removed
- Unused `scanner.rs` module.
- Legacy `Expr::Null` and `StatementResult::Return` variants.