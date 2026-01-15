# RizzScript Language Specification v0.1

A utility-focused, esoteric scripting language for async tasks, networking, and high-performance I/O operations with a vibe-based syntax.

## Overview

**Purpose**: A functional, utility-based scripting language optimized for: 
- Asynchronous task creation and handling
- HTTP/TCP request/response operations
- Fast I/O operations
- SIMD-accelerated JSON parsing
- High-performance regex parsing

**Execution**:  Interpreted runtime (like Python)
```bash
rizz run script.rizz
```

---

## Core Philosophy

- **Functional Programming Paradigm**: First-class functions, immutability encouraged
- **Esoteric but Intuitive**: Urban slang keywords that are memorable and expressive
- **Utility-First**:  Built-in primitives for common async/networking/IO tasks
- **Performance-Oriented**:  SIMD operations and optimized parsers under the hood

---

## Data Types

| Type | Description | Example |
|------|-------------|---------|
| `int` | Integer numbers | `42`, `-10` |
| `float` | Floating-point numbers | `3.14`, `-0.5` |
| `char` | Single character | `'a'`, `'z'` |
| `string` | Text sequences | `"Hello World"` |
| `object` | Key-value store | `{"name": "rizz", "age": 21}` |
| `array` | Ordered collections | `[1, 2, 3, "yo"]` |

---

## Keywords & Syntax

### Variables & Constants

```rizz
Ayo name = "RizzScript"           // Mutable variable
Yoo PI = 3.14159                  // Immutable constant
Ayo count = 0
Ayo data = {"key": "value"}
Ayo items = [1, 2, 3, 4, 5]
```

**Keywords**:
- `Ayo` - Declare mutable variable
- `Yoo` - Declare constant (immutable)

---

### Functions

```rizz
Bruh greet(name) {
  Rizz("Hello, " + name)         // Print/return statement
}

Bruh add(a, b) {
  Rizz(a + b)
}

Bruh fetchData(url) HawkTuah {   // Async function
  Ayo response = Spit(url)       // HTTP GET request
  Rizz(response)
}

// Call functions
greet("World")
Ayo sum = add(5, 10)
```

**Keywords**:
- `Bruh` - Function declaration
- `HawkTuah` - Marks function as async
- `Rizz` - Return value / print output
- `Spit` - HTTP request primitive

---

### Conditionals

```rizz
Ayo age = 18

Maybe age >= 18 {
  Rizz("You're valid")
}

Maybe age >= 21 {
  Rizz("Legal everywhere")
} Unless {
  Rizz("Not yet fam")
}

// Ternary-style
Ayo status = Maybe age >= 18 ? "Adult" : "Minor"
```

**Keywords**:
- `Maybe` - If statement
- `Unless` - Else statement

---

### Loops

```rizz
// For loop with range
Crazy i in 0..5 {
  Rizz(i)
}

// For loop with array
Ayo numbers = [1, 2, 3, 4, 5]
Crazy num in numbers {
  Rizz(num * 2)
}

// For loop with condition
Ayo counter = 0
Crazy counter < 10 {
  Rizz(counter)
  counter = counter + 1
}
```

**Keywords**:
- `Crazy` - For loop declaration

---

## Async & Concurrency

### Async Task Creation

```rizz
// Define async function
Bruh downloadFile(url) HawkTuah {
  Ayo response = Spit(url)
  Rizz(response)
}

// Create async task
Ayo task = Vibe downloadFile("https://api.example.com/data")

// Wait for task completion
Ayo result = Chill(task)
Rizz(result)

// Fire and forget
Vibe processData(data)
```

**Keywords**:
- `HawkTuah` - Async function marker
- `Vibe` - Spawn async task
- `Chill` - Await task completion

---

## Networking Primitives

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

// With headers
Ayo headers = {"Authorization": "Bearer token123"}
Ayo response = Spit("https://api.example.com/protected", headers)
```

**Keywords**:
- `Spit` - HTTP GET
- `Yeet` - HTTP POST
- `Flex` - HTTP PUT
- `Ghost` - HTTP DELETE

---

### TCP Operations

```rizz
// Create TCP server
Bruh handleClient(socket) HawkTuah {
  Ayo message = Peek(socket)      // Read from socket
  Rizz("Received: " + message)
  Whisper(socket, "Ack")          // Write to socket
  Dip(socket)                     // Close socket
}

Ayo server = Vibe Listen(8080, handleClient)

// TCP client
Bruh connectToServer() HawkTuah {
  Ayo socket = Holla("localhost", 8080)
  Whisper(socket, "Hello server")
  Ayo response = Peek(socket)
  Dip(socket)
  Rizz(response)
}
```

**Keywords**:
- `Listen` - Create TCP server
- `Holla` - Connect to TCP server
- `Peek` - Read from socket
- `Whisper` - Write to socket
- `Dip` - Close connection

---

## I/O Operations

### File Operations

```rizz
// Read file
Ayo content = Snag("data.txt")
Rizz(content)

// Write file
Ayo text = "Hello RizzScript"
Stash("output.txt", text)

// Append to file
KeepAdding("log.txt", "New log entry\n")

// Delete file
Trash("temp.txt")

// Check if file exists
Maybe FileExists("config.rizz") {
  Ayo config = Snag("config.rizz")
} Unless {
  Rizz("Config not found")
}
```

**Keywords**:
- `Snag` - Read file
- `Stash` - Write file
- `KeepAdding` - Append to file
- `Trash` - Delete file
- `FileExists` - Check file existence

---

## Advanced Primitives

### JSON Parsing (SIMD-Accelerated)

```rizz
Ayo jsonString = '{"name": "Rizz", "score": 9000}'

// Parse JSON
Ayo parsed = Decode(jsonString)
Rizz(parsed. name)    // "Rizz"
Rizz(parsed.score)   // 9000

// Stringify JSON
Ayo obj = {"status": "fire", "count": 42}
Ayo jsonStr = Encode(obj)
Rizz(jsonStr)        // {"status":"fire","count":42}
```

**Keywords**:
- `Decode` - Parse JSON string (SIMD optimized)
- `Encode` - Convert object to JSON string

---

### Regex Operations (Fast Parser)

```rizz
// Match regex
Ayo text = "My number is 555-1234"
Ayo pattern = r"\d{3}-\d{4}"
Ayo matches = Hunt(text, pattern)
Rizz(matches[0])     // "555-1234"

// Replace with regex
Ayo censored = Swap(text, r"\d", "X")
Rizz(censored)       // "My number is XXX-XXXX"

// Check if matches
Maybe Matches(text, r"\d{3}-\d{4}") {
  Rizz("Found a phone number!")
}

// Split by regex
Ayo parts = Split("a1b2c3", r"\d")
Rizz(parts)          // ["a", "b", "c"]
```

**Keywords**:
- `Hunt` - Find regex matches
- `Swap` - Regex replace
- `Matches` - Check if matches pattern
- `Split` - Split string by regex

---

## Error Handling

```rizz
Bruh riskyOperation() {
  Maybe something_bad {
    Cringe("Something went wrong!")
  }
  Rizz("All good")
}

// Try-catch style
Ayo result = Attempt {
  Ayo data = Spit("https://api.example.com/data")
  Rizz(data)
} Eww (error) {
  Rizz("Error: " + error)
  Rizz(null)
}
```

**Keywords**:
- `Attempt` - Try block
- `Eww` - Catch block
- `Cringe` - Throw error/panic

---

## Comments

```rizz
// Single line comment

/*
  Multi-line comment
  for longer explanations
*/

Ayo x = 42  // Inline comment
```

---

## Complete Example:  Async HTTP Server

```rizz
// Import core modules (if module system exists)
// For now, assume built-ins are available

Yoo PORT = 3000

Bruh handleRequest(request) HawkTuah {
  Ayo path = request.path
  Ayo method = request.method
  
  Maybe path == "/api/users" && method == "GET" {
    Ayo users = [
      {"id": 1, "name": "Chad"},
      {"id": 2, "name": "Stacy"}
    ]
    Ayo json = Encode(users)
    Rizz({"status": 200, "body": json})
  } Unless Maybe path == "/api/hello" {
    Rizz({"status": 200, "body": "Yo what's good"})
  } Unless {
    Rizz({"status": 404, "body": "Not found fam"})
  }
}

Bruh main() HawkTuah {
  Rizz("Starting server on port " + PORT)
  Ayo server = Listen(PORT, handleRequest)
  Chill(server)  // Keep server running
}

// Run it
Vibe main()
```

---

## Complete Example:  Concurrent File Processing

```rizz
Yoo FILES = ["data1.txt", "data2.txt", "data3.txt"]

Bruh processFile(filename) HawkTuah {
  Rizz("Processing: " + filename)
  Ayo content = Snag(filename)
  
  // Extract emails using regex
  Ayo emails = Hunt(content, r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b")
  
  // Save results
  Ayo output = {"file": filename, "emails": emails}
  Ayo json = Encode(output)
  Stash(filename + ".result. json", json)
  
  Rizz("Done:  " + filename)
}

Bruh main() HawkTuah {
  Ayo tasks = []
  
  // Spawn tasks for each file
  Crazy file in FILES {
    Ayo task = Vibe processFile(file)
    tasks = tasks + [task]
  }
  
  // Wait for all tasks
  Crazy task in tasks {
    Chill(task)
  }
  
  Rizz("All files processed!")
}

Vibe main()
```

---

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

---

## Additional Keywords Reference

| Keyword | Purpose | Category |
|---------|---------|----------|
| `Ayo` | Declare variable | Variables |
| `Yoo` | Declare constant | Variables |
| `Bruh` | Define function | Functions |
| `Rizz` | Return/output | Functions |
| `Maybe` | If condition | Control Flow |
| `Unless` | Else | Control Flow |
| `Crazy` | For loop | Loops |
| `HawkTuah` | Async marker | Concurrency |
| `Vibe` | Spawn async task | Concurrency |
| `Chill` | Await task | Concurrency |
| `Spit` | HTTP GET | Networking |
| `Yeet` | HTTP POST | Networking |
| `Flex` | HTTP PUT | Networking |
| `Ghost` | HTTP DELETE | Networking |
| `Listen` | TCP server | Networking |
| `Holla` | TCP connect | Networking |
| `Peek` | Socket read | Networking |
| `Whisper` | Socket write | Networking |
| `Dip` | Close connection | Networking |
| `Snag` | Read file | I/O |
| `Stash` | Write file | I/O |
| `KeepAdding` | Append file | I/O |
| `Trash` | Delete file | I/O |
| `Decode` | Parse JSON | Parsing |
| `Encode` | Stringify JSON | Parsing |
| `Hunt` | Regex find | Regex |
| `Swap` | Regex replace | Regex |
| `Matches` | Regex test | Regex |
| `Split` | Regex split | Regex |
| `Attempt` | Try block | Errors |
| `Eww` | Catch block | Errors |
| `Cringe` | Throw error | Errors |

---

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
- **Destructuring**:  `Ayo {name, age} = user`
- **Spread operators**: `Ayo combined = [... arr1, ...arr2]`
- **Pipe operator**: `data |> parse |> transform |> save`
- **Anonymous functions**: `Ayo add = (a, b) => a + b`

---

## Example File:  `hello.rizz`

````rizz
/*
  RizzScript Hello World
  A simple async HTTP example
*/

Yoo MESSAGE = "What's good, RizzScript!"

Bruh fetchQuote() HawkTuah {
  Attempt {
    Ayo response = Spit("https://api.quotable.io/random")
    Ayo data = Decode(response. body)
    Rizz(data.content)
  } Eww (error) {
    Rizz("Couldn't fetch quote: " + error)
    Rizz("Using fallback instead")
  }
}

Bruh main() HawkTuah {
  Rizz(MESSAGE)
  
  Ayo quote = Chill(Vibe fetchQuote())
  Rizz("Quote of the day: " + quote)
  
  Ayo numbers = [1, 2, 3, 4, 5]
  Ayo doubled = []
  
  Crazy num in numbers {
    doubled = doubled + [num * 2]
  }
  
  Rizz("Doubled numbers: " + Encode(doubled))
}

Vibe main()
````

Run with: 
```bash
rizz run hello.rizz
```

---

This spec provides a solid foundation for RizzScript!  The esoteric keywords make it memorable while the functional paradigm and built-in async/networking primitives make it genuinely useful for modern utility scripting.  🔥
