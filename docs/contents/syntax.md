# Syntax & Keywords

RizzScript uses a unique vibe-based syntax with memorable keywords. This guide covers all syntax elements.

## Variables & Constants

### Declaring Variables

Use `Ayo` to declare mutable variables:

```rizz
Ayo name = "RizzScript"
Ayo count = 0
Ayo data = {"key": "value"}
Ayo items = [1, 2, 3, 4, 5]
```

### Declaring Constants

Use `Yoo` to declare immutable constants:

```rizz
Yoo PI = 3.14159
Yoo MAX_SIZE = 1000
Yoo API_URL = "https://api.example.com"
```

## Functions

### Function Declaration

Use `Bruh` to define functions:

```rizz
Bruh greet(name) {
  Rizz("Hello, " + name)
}

Bruh add(a, b) {
  Rizz(a + b)
}
```

### Return Values

Use `Rizz` to return values or print output:

```rizz
Bruh multiply(x, y) {
  Rizz(x * y)  // Returns x * y
}

Bruh printMessage(msg) {
  Rizz(msg)  // Prints and returns msg
}
```

### Async Functions

Mark functions as async with `HawkTuah`:

```rizz
Bruh fetchData(url) HawkTuah {
  Ayo response = Spit(url)
  Rizz(response)
}
```

## Conditionals

### If Statements

Use `Maybe` for conditional execution:

```rizz
Ayo age = 18

Maybe age >= 18 {
  Rizz("You're valid")
}
```

### If-Else Statements

Use `Unless` for else clauses:

```rizz
Maybe age >= 21 {
  Rizz("Legal everywhere")
} Unless {
  Rizz("Not yet fam")
}
```

### Ternary Expressions

Shorthand conditional expressions:

```rizz
Ayo status = Maybe age >= 18 ? "Adult" : "Minor"
```

## Loops

### Range Loops

Iterate over numeric ranges:

```rizz
Crazy i in 0..5 {
  Rizz(i)  // Prints 0, 1, 2, 3, 4
}
```

### Array Loops

Iterate over arrays:

```rizz
Ayo numbers = [1, 2, 3, 4, 5]
Crazy num in numbers {
  Rizz(num * 2)
}
```

### Conditional Loops

Loop while a condition is true:

```rizz
Ayo counter = 0
Crazy counter < 10 {
  Rizz(counter)
  counter = counter + 1
}
```

## Async & Concurrency

### Spawning Tasks

Use `Vibe` to spawn async tasks:

```rizz
Ayo task = Vibe fetchData("https://api.example.com/data")
```

### Awaiting Tasks

Use `Chill` to wait for task completion:

```rizz
Ayo result = Chill(task)
```

### Fire and Forget

Spawn tasks without awaiting:

```rizz
Vibe processData(data)  // Runs in background
```

## Networking Keywords

### HTTP Operations

| Keyword | Method | Description |
|---------|--------|-------------|
| `Spit` | GET | Fetch data from URL |
| `Yeet` | POST | Send data to URL |
| `Flex` | PUT | Update resource |
| `Ghost` | DELETE | Delete resource |

```rizz
// GET request
Ayo response = Spit("https://api.example.com/users")

// POST request
Ayo data = {"name": "Rizz"}
Ayo response = Yeet("https://api.example.com/users", data)

// PUT request
Ayo updated = Flex("https://api.example.com/users/1", data)

// DELETE request
Ghost("https://api.example.com/users/1")

// With headers
Ayo headers = {"Authorization": "Bearer token123"}
Ayo response = Spit("https://api.example.com/protected", headers)
```

### TCP Operations

| Keyword | Description |
|---------|-------------|
| `Listen` | Create TCP server |
| `Holla` | Connect to TCP server |
| `Peek` | Read from socket |
| `Whisper` | Write to socket |
| `Dip` | Close connection |

```rizz
Bruh handleClient(socket) HawkTuah {
  Ayo message = Peek(socket)
  Whisper(socket, "Ack")
  Dip(socket)
}

Ayo server = Vibe Listen(8080, handleClient)
```

## I/O Operations

### File Operations

| Keyword | Description |
|---------|-------------|
| `Snag` | Read file |
| `Stash` | Write file |
| `KeepAdding` | Append to file |
| `Trash` | Delete file |
| `FileExists` | Check file existence |

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

## JSON Operations

### Parsing JSON

Use `Decode` to parse JSON strings (SIMD-accelerated):

```rizz
Ayo jsonString = '{"name": "Rizz", "score": 9000}'
Ayo parsed = Decode(jsonString)
Rizz(parsed.name)   // "Rizz"
Rizz(parsed.score)  // 9000
```

### Stringifying JSON

Use `Encode` to convert objects to JSON:

```rizz
Ayo obj = {"status": "fire", "count": 42}
Ayo jsonStr = Encode(obj)
Rizz(jsonStr)  // {"status":"fire","count":42}
```

## Regex Operations

### Finding Matches

Use `Hunt` to find regex matches:

```rizz
Ayo text = "My number is 555-1234"
Ayo pattern = r"\d{3}-\d{4}"
Ayo matches = Hunt(text, pattern)
Rizz(matches[0])  // "555-1234"
```

### Replacing

Use `Swap` to replace matches:

```rizz
Ayo censored = Swap(text, r"\d", "X")
Rizz(censored)  // "My number is XXX-XXXX"
```

### Testing Matches

Use `Matches` to check if pattern matches:

```rizz
Maybe Matches(text, r"\d{3}-\d{4}") {
  Rizz("Found a phone number!")
}
```

### Splitting

Use `Split` to split by regex:

```rizz
Ayo parts = Split("a1b2c3", r"\d")
Rizz(parts)  // ["a", "b", "c"]
```

## Error Handling

### Try-Catch Blocks

Use `Attempt` and `Eww` for error handling:

```rizz
Attempt {
  Ayo data = Spit("https://api.example.com/data")
  Rizz(data)
} Eww (error) {
  Rizz("Error: " + error)
  Rizz(null)
}
```

### Throwing Errors

Use `Cringe` to throw errors:

```rizz
Bruh riskyOperation() {
  Maybe something_bad {
    Cringe("Something went wrong!")
  }
  Rizz("All good")
}
```

## Operators

### Arithmetic Operators

```rizz
Ayo a = 10
Ayo b = 3

Rizz(a + b)  // 13 (Addition)
Rizz(a - b)  // 7 (Subtraction)
Rizz(a * b)  // 30 (Multiplication)
Rizz(a / b)  // 3 (Division)
Rizz(a % b)  // 1 (Modulo)
```

### String Concatenation

```rizz
Ayo greeting = "Hello" + " " + "World"
Rizz(greeting)  // "Hello World"
```

### Comparison Operators

```rizz
Ayo x = 10
Ayo y = 20

Rizz(x == y)  // false (Equal)
Rizz(x != y)  // true (Not equal)
Rizz(x > y)   // false (Greater than)
Rizz(x < y)   // true (Less than)
Rizz(x >= y)  // false (Greater or equal)
Rizz(x <= y)  // true (Less or equal)
```

### Logical Operators

```rizz
Ayo a = true
Ayo b = false

Rizz(a && b)  // false (AND)
Rizz(a || b)  // true (OR)
Rizz(!a)      // false (NOT)
```

## Comments

### Single-Line Comments

```rizz
// This is a single-line comment
Ayo x = 42  // Inline comment
```

### Multi-Line Comments

```rizz
/*
  This is a multi-line comment
  for longer explanations
*/
```

## Complete Keyword Reference

| Keyword | Category | Purpose |
|---------|----------|---------|
| `Ayo` | Variables | Declare mutable variable |
| `Yoo` | Variables | Declare immutable constant |
| `Bruh` | Functions | Define function |
| `Rizz` | Functions | Return/output |
| `Maybe` | Control Flow | If condition |
| `Unless` | Control Flow | Else |
| `Crazy` | Loops | For loop |
| `HawkTuah` | Concurrency | Async marker |
| `Vibe` | Concurrency | Spawn async task |
| `Chill` | Concurrency | Await task |
| `Spit` | Networking | HTTP GET |
| `Yeet` | Networking | HTTP POST |
| `Flex` | Networking | HTTP PUT |
| `Ghost` | Networking | HTTP DELETE |
| `Listen` | Networking | TCP server |
| `Holla` | Networking | TCP connect |
| `Peek` | Networking | Socket read |
| `Whisper` | Networking | Socket write |
| `Dip` | Networking | Close connection |
| `Snag` | I/O | Read file |
| `Stash` | I/O | Write file |
| `KeepAdding` | I/O | Append file |
| `Trash` | I/O | Delete file |
| `FileExists` | I/O | Check file existence |
| `Decode` | Parsing | Parse JSON |
| `Encode` | Parsing | Stringify JSON |
| `Hunt` | Regex | Find matches |
| `Swap` | Regex | Replace matches |
| `Matches` | Regex | Test match |
| `Split` | Regex | Split by regex |
| `Attempt` | Errors | Try block |
| `Eww` | Errors | Catch block |
| `Cringe` | Errors | Throw error |
