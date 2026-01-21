# Mutation Syntax

Mutations are the core primitive of Hybrid. They allow you to embed foreign code blocks directly.

## Structure

```hybrid
mutation <runtime_id> {
    // native code here
}
```

## Supported Runtimes

- `python`: CPython 3.10+
- `rust`: Native interaction (zero cost)
- `node`: V8 Isolate (Coming Soon)
