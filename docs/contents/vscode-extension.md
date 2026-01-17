# VS Code Extension

The RizzScript VS Code extension provides syntax highlighting, snippets, and LSP integration.

## Installation

### From VS Code Marketplace

1. Open VS Code
2. Go to Extensions (Ctrl+Shift+X / Cmd+Shift+X)
3. Search for "RizzScript"
4. Click Install

### From VSIX File

1. Download `rizzscript-0.1.0.vsix` from [GitHub Releases](https://github.com/rizz-script/rizz/releases)
2. Open VS Code
3. Go to Extensions
4. Click the `...` menu → "Install from VSIX..."
5. Select the downloaded file

### From Command Line

```bash
code --install-extension rizzscript-0.1.0.vsix
```

## Features

### Syntax Highlighting

Full syntax highlighting for all RizzScript keywords and constructs:

```rizz
Ayo name = "RizzScript"  // Variables highlighted
Bruh greet(name) {       // Functions highlighted
  Rizz("Hello")          // Built-ins highlighted
}
```

### Code Snippets

Useful snippets for common patterns:

- `bruh` - Function declaration
- `maybe` - If statement
- `crazy` - For loop
- `hawktuah` - Async function
- `attempt` - Try-catch block

Type the snippet prefix and press `Tab` to expand.

### Language Server Protocol

Full LSP support including:
- **Go to Definition** (F12)
- **Hover Information**
- **Error Diagnostics**
- **Code Completion**
- **Find References** (Shift+F12)
- **Rename Symbol** (F2)

### File Association

Files with `.rizz` extension are automatically recognized as RizzScript.

## Configuration

### LSP Settings

Configure the LSP server in VS Code settings:

```json
{
  "rizzscript.lsp.path": "",  // Path to rizz-lsp executable
  "rizzscript.lsp.trace.server": "off"  // Trace level
}
```

### Editor Settings

Recommended settings for RizzScript:

```json
{
  "[rizz]": {
    "editor.defaultFormatter": "rizzscript",
    "editor.formatOnSave": true,
    "editor.tabSize": 2
  }
}
```

## Usage

### Opening Files

Simply open any `.rizz` file in VS Code. The extension will automatically activate.

### Formatting

Format your code:
- **Format Document**: Shift+Alt+F (Windows/Linux) or Shift+Option+F (Mac)
- **Format Selection**: Right-click → Format Selection

Or use the command palette:
1. Press Ctrl+Shift+P (Cmd+Shift+P on Mac)
2. Type "Format Document"
3. Press Enter

### Running Scripts

Use the integrated terminal to run scripts:

```bash
rizz run script.rizz
```

Or create a task in `.vscode/tasks.json`:

```json
{
  "version": "2.0.0",
  "tasks": [
    {
      "label": "Run RizzScript",
      "type": "shell",
      "command": "rizz run ${file}",
      "problemMatcher": []
    }
  ]
}
```

Then press Ctrl+Shift+P → "Run Task" → "Run RizzScript"

## Troubleshooting

### Extension Not Activating

1. Check that files have `.rizz` extension
2. Reload VS Code window (Ctrl+R / Cmd+R)
3. Check Output panel → "RizzScript" for errors

### No Syntax Highlighting

1. Ensure file has `.rizz` extension
2. Check that extension is enabled
3. Try reloading VS Code window

### LSP Not Working

1. Check Output panel → "RizzScript Language Server"
2. Verify `rizz-lsp` is installed and in PATH
3. Check LSP settings in VS Code
4. Enable trace logging for debugging

### Snippets Not Working

1. Ensure file has `.rizz` extension
2. Type snippet prefix and press `Tab`
3. Check that snippets are enabled in settings

## Building from Source

```bash
cd apps/rizz-ext
npm install
npm run compile
npm run package
```

The VSIX file will be at `rizzscript-0.1.0.vsix`.

## Contributing

To contribute to the extension:
1. Fork the repository
2. Make changes in `apps/rizz-ext/`
3. Test locally
4. Submit a pull request

See the source code in `apps/rizz-ext/src/` for implementation details.

## Keyboard Shortcuts

| Action | Windows/Linux | Mac |
|--------|---------------|-----|
| Format Document | Shift+Alt+F | Shift+Option+F |
| Go to Definition | F12 | F12 |
| Find References | Shift+F12 | Shift+F12 |
| Rename Symbol | F2 | F2 |
| Show Hover | Hover | Hover |

## Support

For issues and feature requests, please open an issue on [GitHub](https://github.com/rizz-script/rizz/issues).
