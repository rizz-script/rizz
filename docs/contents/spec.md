# Language Specification

The complete RizzScript language specification.

## Overview

**RizzScript** is a utility-focused, esoteric scripting language optimized for:
- Asynchronous task creation and handling
- HTTP/TCP request/response operations
- Fast I/O operations
- SIMD-accelerated JSON parsing
- High-performance regex parsing

**Execution**: Interpreted runtime (like Python)

```bash
rizz run script.rizz
```

## Core Philosophy

- **Functional Programming Paradigm**: First-class functions, immutability encouraged
- **Esoteric but Intuitive**: Urban slang keywords that are memorable and expressive
- **Utility-First**: Built-in primitives for common async/networking/IO tasks
- **Performance-Oriented**: SIMD operations and optimized parsers under the hood

## Data Types

| Type | Description | Example |
|------|-------------|---------|
| `int` | Integer numbers | `42`, `-10` |
| `float` | Floating-point numbers | `3.14`, `-0.5` |
| `char` | Single character | `'a'`, `'z'` |
| `string` | Text sequences | `"Hello World"` |
| `object` | Key-value store | `{"name": "rizz", "age": 21}` |
| `array` | Ordered collections | `[1, 2, 3, "yo"]` |

## Keywords & Syntax

See the [Syntax & Keywords](/syntax) page for detailed documentation.

## Operators

### Arithmetic
- `+` Addition / String concatenation
- `-` Subtraction
- `*` Multiplication
- `/` Division
- `%` Modulo

### Comparison
- `==` Equal
- `!=` Not equal
- `>` Greater than
- `<` Less than
- `>=` Greater or equal
- `<=` Less or equal

### Logical
- `&&` AND
- `||` OR
- `!` NOT

### Assignment
- `=` Assign

## Complete Specification

For the full language specification, see [SPEC.md](https://github.com/rizz-script/rizz/blob/main/SPEC.md) in the repository.

The specification includes:
- Detailed syntax rules
- Type system
- Runtime behavior
- Built-in functions
- Standard library
- Error handling semantics

## Version

Current version: **v0.1**

## Future Considerations

### Potential Additions
- **Module system**: `Bring "http"` (import)
- **Pattern matching**: Enhanced conditionals
- **Streams**: For large file processing
- **WebSocket support**: Real-time communication
- **Database primitives**: Direct DB operations
- **More data structures**: Maps, Sets, Queues
- **Parallel loops**: `CrazyParallel` for SIMD operations

### Syntax Refinements
- **Destructuring**: `Ayo {name, age} = user`
- **Spread operators**: `Ayo combined = [...arr1, ...arr2]`
- **Pipe operator**: `data |> parse |> transform |> save`
- **Anonymous functions**: `Ayo add = (a, b) => a + b`
