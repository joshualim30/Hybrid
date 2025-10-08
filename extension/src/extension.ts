import * as vscode from 'vscode';
import * as path from 'path';

export function activate(context: vscode.ExtensionContext) {
    console.log('Hybrid language extension v0.1.0 is now active!');

    // Register commands
    let runCommand = vscode.commands.registerCommand('hybrid.run', () => {
        runHybridFile();
    });

    let replCommand = vscode.commands.registerCommand('hybrid.repl', () => {
        openREPL();
    });

    context.subscriptions.push(runCommand, replCommand);

    // Register basic completion provider
    let completionProvider = vscode.languages.registerCompletionItemProvider(['hybrid'], {
        provideCompletionItems(document, position, token, context) {
            return provideCompletionItems(document, position, token, context);
        }
    });

    // Register hover provider for basic help
    let hoverProvider = vscode.languages.registerHoverProvider(['hybrid'], {
        provideHover(document, position, token) {
            return provideHover(document, position, token);
        }
    });

    context.subscriptions.push(completionProvider, hoverProvider);
}

// Basic completion provider
function provideCompletionItems(
    document: vscode.TextDocument, 
    position: vscode.Position, 
    token: vscode.CancellationToken, 
    context: vscode.CompletionContext
): vscode.ProviderResult<vscode.CompletionItem[]> {
    const completions: vscode.CompletionItem[] = [];

    // Add keyword completions
    const keywords = ['var', 'const', 'block', 'return', 'speak'];
    for (const keyword of keywords) {
        const item = new vscode.CompletionItem(keyword, vscode.CompletionItemKind.Keyword);
        item.detail = `Hybrid keyword: ${keyword}`;
        completions.push(item);
    }
    
    // Add built-in function completions
    const builtInFunctions = [
        {
            name: 'speak',
            params: ['message'],
            description: 'Outputs a message to the console'
        }
    ];
    
    for (const func of builtInFunctions) {
        const item = new vscode.CompletionItem(func.name, vscode.CompletionItemKind.Function);
        item.detail = func.description;
        item.insertText = new vscode.SnippetString(`${func.name}(${func.params.map((p, i) => `\${${i + 1}:${p}}`).join(', ')})`);
        item.documentation = new vscode.MarkdownString(
            `**${func.name}**\n\n${func.description}`
        );
        completions.push(item);
    }

    return completions;
}

// Basic hover provider
function provideHover(
    document: vscode.TextDocument, 
    position: vscode.Position, 
    token: vscode.CancellationToken
): vscode.ProviderResult<vscode.Hover> {
    const range = document.getWordRangeAtPosition(position);
    if (!range) return;

    const word = document.getText(range);
    
    const help: { [key: string]: string } = {
        'var': '**var** - Declares a variable\n\n```hybrid\nvar x = 10;\n```',
        'const': '**const** - Declares a constant\n\n```hybrid\nconst pi = 3.14;\n```',
        'block': '**block** - Defines a function\n\n```hybrid\nblock add(a, b) {\n    return a + b;\n}\n```',
        'return': '**return** - Returns a value from a function\n\n```hybrid\nreturn result;\n```',
        'speak': '**speak** - Outputs a message\n\n```hybrid\nspeak("Hello, World!");\n```'
    };

    const helpText = help[word];
    if (helpText) {
        return new vscode.Hover(new vscode.MarkdownString(helpText), range);
    }

    return undefined;
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

export function deactivate() {}

