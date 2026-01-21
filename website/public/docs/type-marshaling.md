# Type Marshaling

Hybrid's secret sauce is its **zero-copyish** type marshaling system. We try to be clever about how we move data between memory spaces (Rust/Python/Node), favoring speed over absolute safety where possible, but always keeping it typed.

## Primitives

Primitives are passed by value with minimal overhead.

| Hybrid Type | Rust Type | Python Type | Node Type |
| :--- | :--- | :--- | :--- |
| `string` | `String` | `str` | `string` |
| `int` | `i64` | `int` | `BigInt` |
| `float` | `f64` | `float` | `number` |
| `bool` | `bool` | `bool` | `boolean` |

```rust
// In Rust
fn calculate_risk(score: i64) -> bool {
    score > 9000
}
```

```python
# In Python
def format_currency(amount: float) -> str:
    return f"${amount:.2f}"
```

## Complex Objects

Lists and Maps are serialized to BSON (Binary JSON) for transport. This is faster than standard JSON text serialization but still incurs a copy cost.

```hybrid
struct User {
    string name
    int age
    list<string> roles
}
```

When passing a `User` struct to Python, it becomes a `dict` or a `dataclass` (if using our py-helper). In Rust, it becomes a `serde_json::Value` or a derived struct.

## The "Zero-Copy" Promise

Did we lie? Technically, yes. True zero-copy across process boundaries (e.g. to Python subprocesses) is impossible without shared memory (arrow/shm), which we are adding in v0.2.

For now, "Zero-Copy" refers to our **internal** passing mechanism within the Rust Orchestrator, where we move ownership of pointers rather than cloning data whenever possible.

> [!WARNING]
> Large binary blobs >10MB should be passed via file reference or shared memory handles, NOT as raw byte strings, to avoid serialization lag.
