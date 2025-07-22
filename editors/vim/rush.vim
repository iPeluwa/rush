" Vim syntax file for TaskRush .rush files
" Language: TaskRush Configuration
" Maintainer: TaskRush Team
" Latest Revision: 2025-07-22

if exists("b:current_syntax")
  finish
endif

" Include YAML syntax as base
runtime! syntax/yaml.vim

" TaskRush specific keywords
syn keyword rushKeyword tasks cmd description depends_on cache_files env contained
syn match rushTaskName /^\s*[a-zA-Z0-9_-]\+:/ contains=rushKeyword

" Highlight task names differently
hi def link rushTaskName Function
hi def link rushKeyword Keyword

let b:current_syntax = "rush"

" Set filetype detection
autocmd BufNewFile,BufRead *.rush set filetype=rush
