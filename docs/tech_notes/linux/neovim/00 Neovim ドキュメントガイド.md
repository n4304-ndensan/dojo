# Neovim ドキュメントガイド

## この文書の責務

Ubuntu 24.04 上に Neovim 0.11 系の開発環境を構築する一連の手順書群への入口です。各文書の責務と読む順序を示します。

## 前提

- OS: Ubuntu 24.04 LTS
- シェル: **zsh**（[[00 zsh ドキュメントガイド]] を先に完了しておくこと）
- Neovim: 0.11.5（公式 `.deb`）
- ツールチェーン: Python は **uv**、Rust は **cargo**（rustup 経由）、C# は **dotnet**
- 設計方針: Neovim 0.11 のネイティブ機能を優先し、プラグインの部品点数を最小化する

## 文書一覧と読む順序

| # | 文書 | 責務 |
| --- | --- | --- |
| 1 | [[01 Ubuntu Neovim 環境 OS 基盤セットアップ]] | OS パッケージ、言語ツールチェーン、Neovim 本体のインストール |
| 2 | [[02 Neovim コア設定]] | lazy.nvim の導入、options、keymaps、ディレクトリ構造 |
| 3 | [[03 プラグイン設定（検索・ファイル・UX）]] | fzf-lua、oil.nvim、snacks.nvim、gitsigns.nvim、which-key |
| 4 | [[04 LSP・補完・整形]] | nvim-lspconfig、mason.nvim、blink.cmp、conform.nvim、treesitter |
| 5 | [[05 言語別設定]] | C#(roslyn.nvim)、Rust(rustaceanvim)、Python(basedpyright)、Markdown |

各文書は独立して読めますが、初回セットアップでは 1 → 5 の順に進めてください。

## 構成全体像

```text
fzf-lua ──── 検索（files / grep / buffers / git）
oil.nvim ─── ファイル操作
snacks.nvim ─ UX 統合（terminal / bigfile / notifier / quickfile）
blink.cmp ── 補完（高速）
conform.nvim  整形（保存時自動）
mason.nvim ── 外部ツール管理（LSP サーバー・フォーマッター）
nvim-lspconfig + vim.lsp.enable() ── LSP 設定（0.11 ネイティブ方式）
```

言語別:

```text
C#     → roslyn.nvim + csharpier（dotnet tool）
Rust   → rustaceanvim + rust-analyzer（rustup component）+ rustfmt
Python → basedpyright（uv tool）+ ruff（uv tool）
Markdown → render-markdown.nvim（バッファ内）+ markdown-preview.nvim（ブラウザ）
```

## 設定ファイルの配置

```text
~/.config/nvim/
├── init.lua
└── lua/
    ├── config/
    │   ├── lazy.lua
    │   ├── options.lua
    │   └── keymaps.lua
    └── plugins/
        ├── core.lua
        ├── lsp.lua
        ├── csharp.lua
        └── lang-extra.lua
```

## 参考

- [Neovim 公式リリース](https://github.com/neovim/neovim-releases/releases)
- [lazy.nvim](https://github.com/folke/lazy.nvim)
- [Neovim 0.11 LSP 移行ガイド（nvim-lspconfig wiki）](https://github.com/neovim/nvim-lspconfig)
