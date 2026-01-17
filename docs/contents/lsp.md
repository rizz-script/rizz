# Language Server Protocol (LSP)

RizzScript includes a Language Server Protocol implementation for IDE integration.

## What is LSP?

The Language Server Protocol (LSP) enables rich code editing features like:
- **Go to Definition** - Jump to where symbols are defined
- **Hover Information** - See documentation and type information
- **Error Diagnostics** - Real-time error checking
- **Code Completion** - Intelligent autocomplete
- **Symbol Search** - Find references and definitions
- **Code Formatting** - Format code on save

## Installation

The LSP server is included with the VS Code extension. If you want to use it with other editors, you can build it separately:

```bash
cd apps/rizz-lsp
cargo build --release
```

The binary will be at `target/release/rizz-lsp`.

## VS Code Integration

The LSP is automatically enabled when you install the RizzScript VS Code extension. No additional configuration needed!

### Configuration

You can configure the LSP in VS Code settings:

```json
{
  "rizzscript.lsp.path": "",  // Path to rizz-lsp executable (empty = bundled)
  "rizzscript.lsp.trace.server": "off"  // "off" | "messages" | "verbose"
}
```

### Trace Server Communication

Enable tracing to debug LSP communication:

1. Open VS Code Settings
2. Search for "rizzscript.lsp.trace.server"
3. Set to "messages" or "verbose"
4. Check Output panel → "RizzScript Language Server"

## Features

### Go to Definition

Right-click on a symbol and select "Go to Definition" or press `F12`.

### Hover Information

Hover over any symbol to see its type and documentation.

### Error Diagnostics

Errors are shown in real-time with red squiggles. Check the Problems panel for all errors.

### Code Completion

Type `.` after an object or array to see available properties and methods.

### Find References

Right-click on a symbol and select "Find All References" or press `Shift+F12`.

### Rename Symbol

Right-click on a symbol and select "Rename Symbol" or press `F2`.

## Using with Other Editors

### Neovim

Using [nvim-lspconfig](https://github.com/neovim/nvim-lspconfig):

```lua
require('lspconfig').rizz_lsp.setup({
  cmd = { 'rizz-lsp' },
  filetypes = { 'rizz' },
  root_dir = function(fname)
    return vim.fn.getcwd()
  end,
})
```

### Emacs

Using [lsp-mode](https://github.com/emacs-lsp/lsp-mode):

```elisp
(require 'lsp-mode)
(add-to-list 'lsp-language-id-configuration '(rizz-mode . "rizz"))

(lsp-register-client
 (make-lsp-client
  :new-connection (lsp-stdio-connection "rizz-lsp")
  :activation-fn (lsp-activate-on "rizz")
  :server-id 'rizz-lsp))
```

### Vim

Using [vim-lsp](https://github.com/prabirshrestha/vim-lsp):

```vim
if executable('rizz-lsp')
  au User lsp_setup call lsp#register_server({
    \ 'name': 'rizz-lsp',
    \ 'cmd': {server_info->['rizz-lsp']},
    \ 'whitelist': ['rizz'],
    \ })
endif
```

## Troubleshooting

### LSP Not Starting

1. Check that `rizz-lsp` is in your PATH
2. Check VS Code Output panel for errors
3. Enable trace logging to see detailed communication

### No Autocomplete

1. Ensure the file has a `.rizz` extension
2. Check that the LSP server is running (check Output panel)
3. Try restarting VS Code

### Errors Not Showing

1. Check Problems panel (View → Problems)
2. Ensure file is saved
3. Check LSP trace output for errors

## Building from Source

```bash
cd apps/rizz-lsp
cargo build --release
```

The binary will be at `target/release/rizz-lsp`.

## Contributing

To contribute to the LSP implementation, see the source code in `apps/rizz-lsp/src/`.
