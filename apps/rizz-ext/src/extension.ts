import * as path from 'path';
import * as vscode from 'vscode';
import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
  TransportKind,
} from 'vscode-languageclient/node';

let client: LanguageClient;

export async function activate(context: vscode.ExtensionContext) {
  console.log('RizzScript extension is now active!');

  // Get LSP server path from configuration
  const config = vscode.workspace.getConfiguration('rizzscript');
  let serverPath = config.get<string>('lsp.path');
  let turnOnTrace = config.get<boolean>('lsp.trace.server');

  // If no custom path, try to find rizz-lsp in PATH or workspace
  if (!serverPath) {
    // Try to find in workspace cargo target
    const workspaceFolders = vscode.workspace.workspaceFolders;
    if (workspaceFolders && workspaceFolders.length > 0) {
      const workspaceRoot = workspaceFolders[0].uri.fsPath;
      const debugPath = path.join(workspaceRoot, 'target', 'debug', 'rizz-lsp');
      const releasePath = path.join(workspaceRoot, 'target', 'release', 'rizz-lsp');
      
      // Check if built LSP exists
      const fs = require('fs');
      if (fs.existsSync(releasePath)) {
        serverPath = releasePath;
      } else if (fs.existsSync(debugPath)) {
        serverPath = debugPath;
      }
    }

    // Fallback to system PATH
    if (!serverPath) {
      serverPath = 'rizz-lsp';
    }
  }

  // LSP server options
  const serverOptions: ServerOptions = {
    command: serverPath,
    transport: TransportKind.stdio,
  };

  // LSP client options
  const clientOptions: LanguageClientOptions = {
    documentSelector: [{ scheme: 'file', language: 'rizz' }],
    synchronize: {
      fileEvents: vscode.workspace.createFileSystemWatcher('**/*.rizz'),
    },
  };

  // Create the language client
  client = new LanguageClient(
    'rizzscript',
    'RizzScript Language Server',
    serverOptions,
    clientOptions
  );

  // Start the client (this will also launch the server)
  try {
    await client.start();
    vscode.window.showInformationMessage('RizzScript LSP started successfully!');
    if (turnOnTrace) {
      console.log('LSP trace is on');
    }
  } catch (error) {
    vscode.window.showErrorMessage(
      `Failed to start RizzScript LSP: ${error}. Make sure rizz-lsp is built and available.`
    );
    console.error('Failed to start LSP:', error);
  }

  // Register commands
  context.subscriptions.push(
    vscode.commands.registerCommand('rizzscript.restartServer', async () => {
      if (client) {
        await client.stop();
        await client.start();
        vscode.window.showInformationMessage('RizzScript LSP restarted');
      }
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand('rizzscript.runScript', async () => {
      const editor = vscode.window.activeTextEditor;
      if (!editor || editor.document.languageId !== 'rizz') {
        vscode.window.showErrorMessage('No RizzScript file is active');
        return;
      }

      const filePath = editor.document.uri.fsPath;
      const terminal = vscode.window.createTerminal('RizzScript');
      terminal.show();
      terminal.sendText(`cargo run --bin rizz -- run "${filePath}"`);
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand('rizzscript.formatScript', async () => {
      const editor = vscode.window.activeTextEditor;
      if (!editor || editor.document.languageId !== 'rizz') {
        vscode.window.showErrorMessage('No RizzScript file is active');
        return;
      }

      const filePath = editor.document.uri.fsPath;
      const terminal = vscode.window.createTerminal('RizzScript');
      terminal.show();
      terminal.sendText(`cargo run --bin rizz -- format "${filePath}"`);
    })
  );
}

export function deactivate(): Thenable<void> | undefined {
  if (!client) {
    return undefined;
  }
  return client.stop();
}
