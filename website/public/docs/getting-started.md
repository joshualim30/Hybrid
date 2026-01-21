# Getting Started

Welcome to Hybrid, the runtime for the modern polyglot stack.

## Installation

```bash
curl -sSL https://devhybrid.org/install.sh | sh
```

## Your First Mutation

Hybrid uses `mutation` blocks to define foreign code.

```hybrid
mutation python {
    def hello(name):
        return f"Hello, {name} from Python!"
}

let msg = python::hello("Hybrid User")
print(msg)
```
