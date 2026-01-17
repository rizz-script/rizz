# Examples

Learn RizzScript through practical examples.

## Hello World

The classic first program:

```rizz
Yoo MESSAGE = "What's good, RizzScript!"

Bruh main() {
  Rizz(MESSAGE)
}

Vibe main()
```

## Async HTTP Request

Fetch data from an API:

```rizz
Bruh fetchQuote() HawkTuah {
  Attempt {
    Ayo response = Spit("https://api.quotable.io/random")
    Ayo data = Decode(response.body)
    Rizz(data.content)
  } Eww (error) {
    Rizz("Couldn't fetch quote: " + error)
    Rizz("Using fallback instead")
  }
}

Bruh main() HawkTuah {
  Rizz("Fetching quote...")
  Ayo quote = Chill(Vibe fetchQuote())
  Rizz("Quote of the day: " + quote)
}

Vibe main()
```

## GitHub Profile Fetcher

Fetch and display GitHub user information:

```rizz
Bruh main() HawkTuah {
  Maybe Len(ARGS) < 1 {
    Rizz("usage: rizz run examples/github_profile.rizz <username>")
    Rizz(null)
  }

  Ayo username = ARGS[0]
  Ayo url = "https://api.github.com/users/" + username
  Ayo headers = {
    "User-Agent": "rizzscript",
    "Accept": "application/vnd.github+json"
  }

  Attempt {
    Ayo resp = Spit(url, headers)
    Ayo data = Decode(resp.body)

    Rizz("login: " + data.login)
    Rizz("name: " + data.name)
    Rizz("public_repos: " + data.public_repos)
    Rizz("followers: " + data.followers)
    Rizz("following: " + data.following)
    Rizz("url: " + data.html_url)
    Rizz("bio: " + data.bio)
  } Eww (err) {
    Rizz("error: " + err)
  }
}

Vibe main()
```

Run with:

```bash
rizz run examples/github_profile.rizz bravo68web
```

## File Processing

Read, process, and write files:

```rizz
Bruh processFile(filename) HawkTuah {
  Rizz("Processing: " + filename)
  
  Maybe FileExists(filename) {
    Ayo content = Snag(filename)
    
    // Extract emails using regex
    Ayo emails = Hunt(content, r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b")
    
    // Save results
    Ayo output = {"file": filename, "emails": emails}
    Ayo json = Encode(output)
    Stash(filename + ".result.json", json)
    
    Rizz("Found " + Len(emails) + " emails")
  } Unless {
    Rizz("File not found: " + filename)
  }
}

Bruh main() HawkTuah {
  Ayo files = ["data1.txt", "data2.txt", "data3.txt"]
  
  Crazy file in files {
    Vibe processFile(file)
  }
  
  Rizz("All files queued for processing")
}

Vibe main()
```

## Concurrent File Processing

Process multiple files concurrently:

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
  Stash(filename + ".result.json", json)
  
  Rizz("Done: " + filename)
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

## Simple HTTP Server

Create a basic HTTP server:

```rizz
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

Vibe main()
```

## Array Operations

Working with arrays:

```rizz
Bruh main() HawkTuah {
  Ayo numbers = [1, 2, 3, 4, 5]
  Ayo doubled = []
  
  Crazy num in numbers {
    doubled = doubled + [num * 2]
  }
  
  Rizz("Original: " + Encode(numbers))
  Rizz("Doubled: " + Encode(doubled))
  
  // Filter even numbers
  Ayo evens = []
  Crazy num in numbers {
    Maybe num % 2 == 0 {
      evens = evens + [num]
    }
  }
  
  Rizz("Evens: " + Encode(evens))
}

Vibe main()
```

## JSON Processing

Parse and manipulate JSON data:

```rizz
Bruh main() HawkTuah {
  Ayo jsonString = '{"users": [{"name": "Alice", "age": 30}, {"name": "Bob", "age": 25}]}'
  Ayo data = Decode(jsonString)
  
  Rizz("Total users: " + Len(data.users))
  
  Crazy user in data.users {
    Rizz(user.name + " is " + user.age + " years old")
  }
  
  // Modify and encode back
  Ayo newUser = {"name": "Charlie", "age": 28}
  data.users = data.users + [newUser]
  
  Ayo updatedJson = Encode(data)
  Stash("users.json", updatedJson)
  Rizz("Updated JSON saved")
}

Vibe main()
```

## Regex Text Processing

Extract and manipulate text with regex:

```rizz
Bruh main() HawkTuah {
  Ayo text = "Contact us at support@example.com or sales@example.com. Call 555-1234 or 555-5678."
  
  // Extract emails
  Ayo emails = Hunt(text, r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b")
  Rizz("Emails found: " + Encode(emails))
  
  // Extract phone numbers
  Ayo phones = Hunt(text, r"\d{3}-\d{4}")
  Rizz("Phone numbers: " + Encode(phones))
  
  // Censor phone numbers
  Ayo censored = Swap(text, r"\d{3}-\d{4}", "XXX-XXXX")
  Rizz("Censored: " + censored)
  
  // Split by spaces
  Ayo words = Split(text, r"\s+")
  Rizz("Word count: " + Len(words))
}

Vibe main()
```

## Error Handling

Robust error handling example:

```rizz
Bruh fetchWithRetry(url, maxRetries) HawkTuah {
  Ayo retries = 0
  
  Crazy retries < maxRetries {
    Attempt {
      Ayo response = Spit(url)
      Rizz(response)
    } Eww (error) {
      retries = retries + 1
      Rizz("Attempt " + retries + " failed: " + error)
      
      Maybe retries >= maxRetries {
        Cringe("Max retries reached")
      }
      
      // Wait before retry
      Chill(Vibe Sleep(1000))
    }
  }
}

Bruh main() HawkTuah {
  Attempt {
    Ayo data = Chill(Vibe fetchWithRetry("https://api.example.com/data", 3))
    Rizz("Success: " + data)
  } Eww (error) {
    Rizz("Final error: " + error)
  }
}

Vibe main()
```

## More Examples

Check out the `examples/` directory in the repository for more examples:

- `hello.rizz` - Basic hello world
- `github_profile.rizz` - GitHub API integration
- `rest_api.rizz` - REST API client
- `grep.rizz` - File search utility
- `redis_server.rizz` - Redis-like server
- `fs_shell_smoke.rizz` - File system operations
- `mod_math.rizz` - Math operations

Run any example:

```bash
rizz run examples/hello.rizz
```
