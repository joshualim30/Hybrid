# Security Guards

Hybrid is secure by default. Foreign code runs in isolated contexts.

## Sandboxing

- **Rust**: Compiled in a temporary workspace.
- **Python**: Verified against a virtual environment whitelist.

## Capabilities

You can restrict what mutations can do via the `Hybrid.toml` config:

```toml
[security]
allow_net = false
allow_fs_write = ["./logs"]
```
