export const EXAMPLES: Record<string, string> = {
  hello: `// RizzScript Hello World
Rizz("Hello, RizzScript Playground!")

Yoo name = "Developer"
Rizz("Welcome, " + name)

// Simple math
Ayo x = 10
Ayo y = 20
Rizz("Sum: " + (x + y))`,

  variables: `// Variables and Constants
Ayo count = 0        // Mutable variable
Yoo PI = 3.14159    // Immutable constant

count = count + 1
Rizz("Count: " + count)
Rizz("PI: " + PI)

// Type annotations
Ayo name: string = "RizzScript"
Ayo age: int = 1
Ayo isActive: bool = true

Rizz("Name: " + name)
Rizz("Age: " + age)`,

  functions: `// Function Definitions
Bruh greet(name) {
  Rizz("Hello, " + name + "!")
}

// Function with return type
Bruh add(a: int, b: int): int {
  Rizz(a + b)
}

// Call functions
greet("RizzScript")
Ayo result = add(5, 10)
Rizz("Result: " + result)

// Function with multiple parameters
Bruh calculate(x, y, operation) {
  Maybe operation == "add" {
    Rizz(x + y)
  } Unless Maybe operation == "multiply" {
    Rizz(x * y)
  } Unless {
    Rizz("Unknown operation")
  }
}

calculate(5, 3, "add")
calculate(5, 3, "multiply")`,

  async: `// Async Functions with HawkTuah
Bruh processData(id) HawkTuah {
  Rizz("Processing: " + id)
  Rizz("Done!")
}

// Spawn async tasks with Vibe
Bruh main() HawkTuah {
  Rizz("Starting async operations...")
  
  Ayo task1 = Vibe processData(1)
  Ayo task2 = Vibe processData(2)
  
  // Await with Chill
  Chill(task1)
  Chill(task2)
  
  Rizz("All tasks completed!")
}

Vibe main()`,

  collections: `// Arrays and Objects
Ayo numbers = [1, 2, 3, 4, 5]
Rizz("Numbers: " + Encode(numbers))

// Iterate with Crazy
Ayo doubled = []
Crazy num in numbers {
  doubled = doubled + [num * 2]
}
Rizz("Doubled: " + Encode(doubled))

// Objects
Ayo person = {
  "name": "RizzScript",
  "version": "1.0",
  "awesome": true
}

Rizz("Person: " + Encode(person))
Rizz("Name: " + person.name)

// Array of objects
Ayo users = [
  {"name": "Alice", "age": 25},
  {"name": "Bob", "age": 30}
]

Crazy user in users {
  Rizz(user.name + " is " + user.age + " years old")
}`,
};

export const DEFAULT_CODE = EXAMPLES.hello;
