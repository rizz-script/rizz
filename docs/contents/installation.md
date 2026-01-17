# Installation

Install RizzScript on your system using one of the methods below.

## From Source (Recommended)

### Prerequisites

- **Rust 1.82+** and Cargo
- **Git**

### Build Steps

1. Clone the repository:

```bash
git clone https://github.com/rizz-script/rizz.git
cd rizz
```

2. Build the interpreter:

```bash
cargo build --release
```

3. Install globally:

```bash
cargo install --path apps/rizz
```

4. Verify installation:

```bash
rizz --version
```

## Using Cargo Install

If the package is published to crates.io:

```bash
cargo install rizz
```

## Pre-built Binaries

Pre-built binaries for common platforms are available in [GitHub Releases](https://github.com/rizz-script/rizz/releases).

### Linux

```bash
# Download and extract
wget https://github.com/rizz-script/rizz/releases/latest/download/rizz-x86_64-unknown-linux-gnu.tar.gz
tar -xzf rizz-x86_64-unknown-linux-gnu.tar.gz
sudo mv rizz /usr/local/bin/
```

### macOS

```bash
# Download and extract
wget https://github.com/rizz-script/rizz/releases/latest/download/rizz-x86_64-apple-darwin.tar.gz
tar -xzf rizz-x86_64-apple-darwin.tar.gz
sudo mv rizz /usr/local/bin/
```

### Windows

1. Download `rizz-x86_64-pc-windows-msvc.zip` from releases
2. Extract and add to PATH

## VS Code Extension

Install the RizzScript VS Code extension for syntax highlighting and LSP support:

1. Open VS Code
2. Go to Extensions (Ctrl+Shift+X / Cmd+Shift+X)
3. Search for "RizzScript"
4. Click Install

Or install from the command line:

```bash
code --install-extension rizzscript-0.1.0.vsix
```

## LSP Server

The LSP server is included with the VS Code extension, or you can build it separately:

```bash
cd apps/rizz-lsp
cargo build --release
```

The LSP server binary will be at `target/release/rizz-lsp`.

## Verify Installation

Run a test script:

```bash
rizz run examples/hello.rizz
```

You should see output like:

```
What's good, RizzScript!
```

## Troubleshooting

### Command not found

Make sure the `rizz` binary is in your PATH. On Unix systems:

```bash
echo $PATH
which rizz
```

### Build errors

Ensure you have Rust 1.82+:

```bash
rustc --version
```

Update Rust if needed:

```bash
rustup update
```

### Permission denied

On Unix systems, you may need to use `sudo` when installing to system directories:

```bash
sudo cargo install --path apps/rizz
```
