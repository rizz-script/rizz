# Getting Started

Welcome to RizzScript! This guide will help you get started with the language.

## What is RizzScript?

RizzScript is a utility-focused, esoteric scripting language optimized for:
- **Asynchronous task creation and handling**
- **HTTP/TCP request/response operations**
- **Fast I/O operations**
- **SIMD-accelerated JSON parsing**
- **High-performance regex parsing**

With a vibe-based syntax using urban slang keywords, RizzScript makes async programming fun and intuitive.

## Quick Start

### Installation

Install RizzScript using Cargo:

```bash
cargo install --path apps/rizz
```

Or build from source:

```bash
cargo build --release
```

### Your First Script

Create a file called `hello.rizz`:

```rizz
Yoo MESSAGE = "What's good, RizzScript!"

Bruh main() {
  Rizz(MESSAGE)
}

Vibe main()
```

Run it:

```bash
rizz run hello.rizz
```

### Async HTTP Example

Here's a more practical example that fetches data from an API:

```rizz
Bruh fetchQuote() HawkTuah {
  Attempt {
    Ayo response = Spit("https://api.quotable.io/random")
    Ayo data = Decode(response.body)
    Rizz(data.content)
  } Eww (error) {
    Rizz("Couldn't fetch quote: " + error)
  }
}

Bruh main() HawkTuah {
  Ayo quote = Chill(Vibe fetchQuote())
  Rizz("Quote: " + quote)
}

Vibe main()
```

## Key Concepts

### Variables and Constants

- `Ayo` - Declare a mutable variable
- `Yoo` - Declare an immutable constant

```rizz
Ayo name = "RizzScript"  // Mutable
Yoo PI = 3.14159         // Immutable
```

### Functions

- `Bruh` - Function declaration
- `HawkTuah` - Marks function as async
- `Rizz` - Return value / print output

```rizz
Bruh greet(name) {
  Rizz("Hello, " + name)
}

Bruh fetchData(url) HawkTuah {
  Ayo response = Spit(url)
  Rizz(response)
}
```

### Async Operations

- `Vibe` - Spawn async task
- `Chill` - Await task completion

```rizz
Ayo task = Vibe fetchData("https://api.example.com/data")
Ayo result = Chill(task)
```

## Next Steps

- Learn about [Syntax and Keywords](/syntax)
- Explore [Examples](/examples)
- Read the [Language Specification](/spec)
- Set up [VS Code Extension](/vscode-extension)
- Configure [LSP Server](/lsp)
