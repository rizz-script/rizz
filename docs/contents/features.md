# Features

RizzScript is designed with modern utility scripting in mind. Here are its key features:

## 🚀 Async-First Design

Built-in async/await primitives make concurrent programming intuitive:

```rizz
Bruh fetchData(url) HawkTuah {
  Ayo response = Spit(url)
  Rizz(response)
}

Ayo task = Vibe fetchData("https://api.example.com/data")
Ayo result = Chill(task)
```

## 🌐 Networking Primitives

First-class support for HTTP and TCP operations:

### HTTP Operations

```rizz
// GET request
Ayo response = Spit("https://api.example.com/users")

// POST request
Ayo data = {"name": "Rizz", "level": 100}
Ayo response = Yeet("https://api.example.com/users", data)

// PUT request
Ayo updated = Flex("https://api.example.com/users/1", data)

// DELETE request
Ghost("https://api.example.com/users/1")
```

### TCP Operations

```rizz
Bruh handleClient(socket) HawkTuah {
  Ayo message = Peek(socket)      // Read from socket
  Whisper(socket, "Ack")          // Write to socket
  Dip(socket)                     // Close socket
}

Ayo server = Vibe Listen(8080, handleClient)
```

## ⚡ Performance Optimizations

- **SIMD-accelerated JSON parsing** for blazing-fast data processing
- **High-performance regex engine** for pattern matching
- **Optimized I/O operations** for file handling

```rizz
Ayo jsonString = '{"name": "Rizz", "score": 9000}'
Ayo parsed = Decode(jsonString)  // SIMD-accelerated

Ayo matches = Hunt(text, r"\d{3}-\d{4}")  // Fast regex
```

## 📁 File Operations

Comprehensive file I/O primitives:

```rizz
// Read file
Ayo content = Snag("data.txt")

// Write file
Stash("output.txt", "Hello RizzScript")

// Append to file
KeepAdding("log.txt", "New entry\n")

// Delete file
Trash("temp.txt")

// Check existence
Maybe FileExists("config.rizz") {
  Ayo config = Snag("config.rizz")
}
```

## 🎨 Vibe-Based Syntax

Memorable keywords inspired by urban slang make code expressive and fun:

- `Ayo` - Variables
- `Bruh` - Functions
- `Maybe` - Conditionals
- `Crazy` - Loops
- `HawkTuah` - Async
- `Vibe` - Spawn tasks
- `Chill` - Await

## 🔧 Developer Experience

### VS Code Extension

- Syntax highlighting
- Code snippets
- LSP integration
- Auto-completion

### Language Server Protocol

Full LSP support for:
- Go to definition
- Hover information
- Error diagnostics
- Code completion

### Formatting

Built-in formatter for consistent code style:

```bash
rizz format examples/
```

### Watch Mode

Auto-restart scripts on file changes:

```bash
rizz run script.rizz --watch
```

## 🛡️ Error Handling

Robust error handling with try-catch style blocks:

```rizz
Attempt {
  Ayo data = Spit("https://api.example.com/data")
  Rizz(data)
} Eww (error) {
  Rizz("Error: " + error)
  Rizz(null)
}
```

## 📊 Data Types

Rich set of built-in types:

- `int` - Integers
- `float` - Floating-point numbers
- `char` - Single characters
- `string` - Text sequences
- `object` - Key-value stores
- `array` - Ordered collections

## 🔍 Regex Support

Powerful regex operations:

```rizz
Ayo text = "My number is 555-1234"
Ayo matches = Hunt(text, r"\d{3}-\d{4}")      // Find matches
Ayo replaced = Swap(text, r"\d", "X")         // Replace
Maybe Matches(text, r"\d{3}-\d{4}") {         // Test match
  Rizz("Found!")
}
Ayo parts = Split("a1b2c3", r"\d")            // Split
```

## 🎯 Functional Programming

First-class functions and immutability encouraged:

```rizz
Bruh map(arr, fn) {
  Ayo result = []
  Crazy item in arr {
    result = result + [fn(item)]
  }
  Rizz(result)
}

Bruh double(x) {
  Rizz(x * 2)
}

Ayo numbers = [1, 2, 3, 4, 5]
Ayo doubled = map(numbers, double)
```

## 🚦 Control Flow

Intuitive control structures:

```rizz
// Conditionals
Maybe age >= 18 {
  Rizz("You're valid")
} Unless {
  Rizz("Not yet fam")
}

// Loops
Crazy i in 0..5 {
  Rizz(i)
}

Crazy num in numbers {
  Rizz(num * 2)
}
```
