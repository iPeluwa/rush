# Editor Support for TaskRush

This directory contains language support files for various code editors to recognize and highlight `.rush` configuration files.

## Visual Studio Code

The complete VS Code extension is in the `../vscode-extension/` directory. It provides:

- Syntax highlighting
- IntelliSense and auto-completion
- Code snippets
- Schema validation
- Task runner integration

**Installation:**
The extension will be published to the VS Code marketplace. For now, you can install it manually:

```bash
cd vscode-extension
npm install
npm run compile
code --install-extension .
```

## Vim/Neovim

Copy the syntax file to your Vim configuration:

```bash
# For Vim
mkdir -p ~/.vim/syntax
cp vim/rush.vim ~/.vim/syntax/

# For Neovim
mkdir -p ~/.config/nvim/syntax
cp vim/rush.vim ~/.config/nvim/syntax/
```

Add to your `.vimrc` or `init.vim`:
```vim
autocmd BufNewFile,BufRead *.rush set filetype=rush
```

## Sublime Text

Copy the syntax definition to your Sublime Text packages:

```bash
# macOS
cp sublime/TaskRush.sublime-syntax ~/Library/Application\ Support/Sublime\ Text/Packages/User/

# Linux
cp sublime/TaskRush.sublime-syntax ~/.config/sublime-text/Packages/User/

# Windows
cp sublime/TaskRush.sublime-syntax %APPDATA%/Sublime\ Text/Packages/User/
```

## GitHub Recognition

The `.gitattributes` file in the root directory tells GitHub to treat `.rush` files as YAML for syntax highlighting.

## Other Editors

For editors not listed here, you can typically configure them to treat `.rush` files as YAML, which will provide basic syntax highlighting for the YAML structure.

### Atom
Add to your `config.cson`:
```cson
"*":
  core:
    customFileTypes:
      "source.yaml": [
        "rush"
      ]
```

### IntelliJ IDEA / WebStorm
1. Go to Settings → Editor → File Types
2. Find "YAML" in the list
3. Add `*.rush` to the file name patterns

### Emacs
Add to your `.emacs` or `init.el`:
```elisp
(add-to-list 'auto-mode-alist '("\\.rush\\'" . yaml-mode))
```

## Language Server Protocol (LSP)

For editors that support LSP, you can configure them to use the YAML language server for `.rush` files with the TaskRush schema for validation.
