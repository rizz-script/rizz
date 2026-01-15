# RizzScript Language Server (LSP)

A Language Server Protocol implementation for RizzScript, providing IDE features like auto-completion, diagnostics, and hover documentation.

## Features

- **Diagnostics**: Real-time syntax error detection using the RizzScript parser
- **Auto-completion**: Keyword and built-in function suggestions
- **Hover Documentation**: Inline documentation for all RizzScript keywords and built-ins
- **Code Snippets**: Common code patterns via completion

## Building

```bash
cargo build --release --package rizz-lsp
```

The binary will be at `target/release/rizz-lsp`.

## Running

The LSP server communicates via stdin/stdout following the LSP protocol:

```bash
rizz-lsp
```

It's designed to be launched by IDE clients (like VSCode) automatically.

## Development

For development builds:

```bash
cargo build --package rizz-lsp
# Binary at target/debug/rizz-lsp
```

## Testing with VSCode

1. Build the LSP server (see above)
2. Install the RizzScript VSCode extension from `apps/rizz-ext`
3. Open a `.rizz` file in VSCode
4. The extension will automatically start the LSP server

## Manual Testing

You can test the LSP manually using tools like:

- [language-server-protocol-inspector](https://github.com/silvanshade/tower-lsp-playground)
- Manual JSON-RPC messages via stdin/stdout

Example initialization request:

```json
Content-Length: 123

{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "initialize",
  "params": {
    "rootUri": "file:///path/to/project",
    "capabilities": {}
  }
}
```

## Supported LSP Features

- `textDocument/didOpen` - Document opened notification
- `textDocument/didChange` - Document changed notification
- `textDocument/completion` - Auto-completion
- `textDocument/hover` - Hover information
- `textDocument/publishDiagnostics` - Syntax error reporting

## Architecture

The LSP uses:

- **tower-lsp**: LSP protocol implementation
- **tokio**: Async runtime
- **rizzscript**: The RizzScript lexer and parser for diagnostics

## Future Enhancements

- Go to definition
- Find references
- Rename symbol
- Document symbols
- Code actions (quick fixes)
- Semantic tokens (better syntax highlighting)
- Formatting support

## License

MIT
