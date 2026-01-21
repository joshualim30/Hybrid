import * as vscode from 'vscode';
import * as path from 'path';
import {
    LanguageClient,
    LanguageClientOptions,
    ServerOptions,
    Executable
} from 'vscode-languageclient/node';

let client: LanguageClient;

export function activate(context: vscode.ExtensionContext) {
    console.log('Hybrid language extension v0.1.0 is now active!');

    // Start Language Server
    const workspaceFolder = vscode.workspace.workspaceFolders?.[0];
    if (workspaceFolder) {
        // Path to the hybrid-lsp executable
        const serverPath = path.join(workspaceFolder.uri.fsPath, 'language', 'target', 'debug', 'hybrid-lsp');

        const run: Executable = {
            command: serverPath,
            options: {
                cwd: workspaceFolder.uri.fsPath
            }
        };

        const serverOptions: ServerOptions = {
            run,
            debug: run
        };

        const clientOptions: LanguageClientOptions = {
            documentSelector: [{ scheme: 'file', language: 'hybrid' }],
            synchronize: {
                fileEvents: vscode.workspace.createFileSystemWatcher('**/*.hyb')
            }
        };

        client = new LanguageClient(
            'hybridLSP',
            'Hybrid Language Server',
            serverOptions,
            clientOptions
        );

        client.start();
    }

    // Register commands
    let runCommand = vscode.commands.registerCommand('hybrid.run', () => {
        runHybridFile();
    });

    let replCommand = vscode.commands.registerCommand('hybrid.repl', () => {
        openREPL();
    });

    let transpileRustCommand = vscode.commands.registerCommand('hybrid.transpile_rust', () => {
        runTranspile('rust');
    });

    let transpilePythonCommand = vscode.commands.registerCommand('hybrid.transpile_python', () => {
        runTranspile('python');
    });

    context.subscriptions.push(runCommand, replCommand, transpileRustCommand, transpilePythonCommand);
}

// Command functions
function runHybridFile() {
    const editor = vscode.window.activeTextEditor;
    if (!editor) {
        vscode.window.showErrorMessage('No active editor');
        return;
    }

    const document = editor.document;
    if (document.languageId !== 'hybrid') {
        vscode.window.showErrorMessage('Not a Hybrid file (.hyb)');
        return;
    }

    document.save().then(() => {
        const filePath = document.fileName;
        const workspaceFolder = vscode.workspace.getWorkspaceFolder(document.uri);

        if (!workspaceFolder) {
            vscode.window.showErrorMessage('No workspace folder found');
            return;
        }

        // Look for the hybrid executable in the language directory
        const hybridPath = path.join(workspaceFolder.uri.fsPath, 'language', 'target', 'debug', 'hybrid');
        const command = `"${hybridPath}" "${filePath}"`;

        const terminal = vscode.window.createTerminal({
            name: 'Hybrid',
            cwd: workspaceFolder.uri.fsPath
        });
        terminal.show();
        terminal.sendText(command);
    });
}

function openREPL() {
    const workspaceFolders = vscode.workspace.workspaceFolders;
    if (!workspaceFolders) {
        vscode.window.showErrorMessage('No workspace folder found');
        return;
    }

    const workspaceFolder = workspaceFolders[0];
    const hybridPath = path.join(workspaceFolder.uri.fsPath, 'language', 'target', 'debug', 'hybrid');

    const terminal = vscode.window.createTerminal({
        name: 'Hybrid REPL',
        cwd: workspaceFolder.uri.fsPath
    });
    terminal.show();
    terminal.sendText(`"${hybridPath}"`);
}

function runTranspile(target: string) {
    const editor = vscode.window.activeTextEditor;
    if (!editor) {
        vscode.window.showErrorMessage('No active editor');
        return;
    }

    const document = editor.document;
    if (document.languageId !== 'hybrid') {
        vscode.window.showErrorMessage('Not a Hybrid file (.hyb)');
        return;
    }

    document.save().then(() => {
        const filePath = document.fileName;
        const workspaceFolder = vscode.workspace.getWorkspaceFolder(document.uri);

        if (!workspaceFolder) {
            vscode.window.showErrorMessage('No workspace folder found');
            return;
        }

        const hybridPath = path.join(workspaceFolder.uri.fsPath, 'language', 'target', 'debug', 'hybrid');
        const ext = target === 'rust' ? 'rs' : 'py';
        const outFile = filePath.replace(/\.hyb$/, `.${ext}`);

        const command = `"${hybridPath}" transpile "${filePath}" ${target} > "${outFile}" && echo "Transpiled to ${path.basename(outFile)}"`;

        const terminal = vscode.window.createTerminal({
            name: `Hybrid Transpile (${target})`,
            cwd: workspaceFolder.uri.fsPath
        });
        terminal.show();
        terminal.sendText(command);
    });
}

export function deactivate(): Thenable<void> | undefined {
    if (!client) {
        return undefined;
    }
    return client.stop();
}
