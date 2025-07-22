import * as vscode from 'vscode';

export function activate(context: vscode.ExtensionContext) {
    console.log('TaskRush language support is now active!');

    // Register hover provider for task definitions
    const hoverProvider = vscode.languages.registerHoverProvider('rush', {
        provideHover(document, position, token) {
            const wordRange = document.getWordRangeAtPosition(position);
            const word = document.getText(wordRange);
            
            // Provide helpful information for TaskRush keywords
            const keywords: { [key: string]: string } = {
                'tasks': 'Root object containing all task definitions',
                'cmd': 'The shell command to execute for this task',
                'description': 'Human-readable description of what this task does',
                'depends_on': 'Array of task names that must complete before this task runs',
                'cache_files': 'Array of file patterns to check for caching this task',
                'env': 'Object containing environment variables to set when running this task'
            };

            if (keywords[word]) {
                return new vscode.Hover(new vscode.MarkdownString(`**${word}**: ${keywords[word]}`));
            }
        }
    });

    // Register completion provider for task properties
    const completionProvider = vscode.languages.registerCompletionItemProvider('rush', {
        provideCompletionItems(document, position, token, context) {
            const line = document.lineAt(position).text;
            const items: vscode.CompletionItem[] = [];

            // If we're in a task context, suggest task properties
            if (line.match(/^\s{4,}/)) {
                const properties = ['cmd', 'description', 'depends_on', 'cache_files', 'env'];
                for (const prop of properties) {
                    const item = new vscode.CompletionItem(prop, vscode.CompletionItemKind.Property);
                    item.insertText = new vscode.SnippetString(`${prop}: $0`);
                    items.push(item);
                }
            }

            return items;
        }
    }, ':');

    // Register command to run TaskRush tasks from the editor
    const runTaskCommand = vscode.commands.registerCommand('taskrush.runTask', async () => {
        const editor = vscode.window.activeTextEditor;
        if (!editor || !editor.document.fileName.endsWith('.rush')) {
            vscode.window.showErrorMessage('Please open a .rush file first');
            return;
        }

        // Parse tasks from the document
        const text = editor.document.getText();
        const taskMatches = text.match(/^\s{2}([a-zA-Z0-9_-]+):/gm);
        
        if (!taskMatches) {
            vscode.window.showErrorMessage('No tasks found in this .rush file');
            return;
        }

        const tasks = taskMatches.map(match => match.trim().slice(0, -1));
        const selectedTask = await vscode.window.showQuickPick(tasks, {
            placeHolder: 'Select a task to run'
        });

        if (selectedTask) {
            const terminal = vscode.window.createTerminal('TaskRush');
            terminal.sendText(`taskrush ${selectedTask}`);
            terminal.show();
        }
    });

    context.subscriptions.push(hoverProvider, completionProvider, runTaskCommand);
}

export function deactivate() {}
