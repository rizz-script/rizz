# RizzScript

> A utility-focused, esoteric scripting language with a vibe-based syntax. Optimized for async tasks, networking, and high-performance I/O.

[![Documentation](https://img.shields.io/badge/docs-available-brightgreen)](https://rizz-script.github.io/rizz/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## 🚀 Quick Start

### Installation

```bash
# Build from source
cargo build --release

# Install globally
cargo install --path apps/rizz
```

### Your First Script

Create `hello.rizz`:

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

## ✨ Features

- 🚀 **Async-First Design** - Built-in async/await primitives
- 🌐 **Networking Primitives** - HTTP (Spit, Yeet, Flex, Ghost) and TCP operations
- ⚡ **High Performance** - SIMD-accelerated JSON parsing, fast regex engine
- 🎨 **Vibe-Based Syntax** - Memorable keywords (Ayo, Bruh, Maybe, Crazy)
- 🔧 **Developer Experience** - VS Code extension with LSP support
- 📁 **Rich I/O** - Comprehensive file operations

## 📚 Documentation

- [Getting Started](https://rizz-script.github.io/rizz/getting-started)
- [Installation Guide](https://rizz-script.github.io/rizz/installation)
- [Syntax & Keywords](https://rizz-script.github.io/rizz/syntax)
- [Examples](https://rizz-script.github.io/rizz/examples)
- [Language Specification](https://rizz-script.github.io/rizz/spec)
- [VS Code Extension](https://rizz-script.github.io/rizz/vscode-extension)
- [LSP Server](https://rizz-script.github.io/rizz/lsp)

## 🛠️ Development

### Build

```bash
# Build interpreter
cargo build

# Build LSP server
cd apps/rizz-lsp && cargo build

# Build VS Code extension
cd apps/rizz-ext && npm install && npm run compile
```

### Run Examples

```bash
# Run hello world
cargo run -- run examples/hello.rizz

# Run with watch mode
cargo run -- run examples/hello.rizz --watch

# Format code
cargo run -- format examples/
```

### Project Structure

```
rizz/
├── apps/
│   ├── rizz/          # CLI interpreter
│   ├── rizz-lsp/      # Language Server Protocol
│   ├── rizz-ext/      # VS Code extension
│   └── rizz-wasm/     # WebAssembly build
├── packages/
│   └── core/          # Core language implementation
├── docs/              # Documentation (VitePress)
└── examples/          # Example scripts
```

## 📦 Components

- **rizz** - CLI interpreter for running RizzScript files
- **rizz-lsp** - Language Server Protocol implementation
- **rizz-ext** - VS Code extension with syntax highlighting and LSP
- **rizz-wasm** - WebAssembly build for browser usage

## 🤝 Contributing

Contributions are welcome! Please read our contributing guidelines and submit pull requests.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🔗 Links

- [Documentation](https://rizz-script.github.io/rizz/)
- [Language Specification](SPEC.md)
- [GitHub Repository](https://github.com/rizz-script/rizz)
- [Issue Tracker](https://github.com/rizz-script/rizz/issues)
