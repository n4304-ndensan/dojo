# LSP・補完・整形

## この文書の責務

Neovim の言語サーバー（LSP）、補完、自動整形、構文解析の設定を扱います。ここで扱うのは **nvim-lspconfig**（LSP 設定）、**mason.nvim**（外部ツール管理）、**blink.cmp**（補完）、**conform.nvim**（整形）、**nvim-treesitter**（構文ハイライト・インデント）です。これらは `lua/plugins/lsp.lua` に定義します。

## 前提

- [[03 プラグイン設定（検索・ファイル・UX）]] が完了していること
- [[01 Ubuntu Neovim 環境 OS 基盤セットアップ]] で外部ツール（basedpyright, ruff, csharpier 等）がインストール済みであること

## Neovim 0.11 での LSP 設定の変化

Neovim 0.11 では LSP の設定方法が大きく変わりました。旧来の `require('lspconfig').xxx.setup({})` 方式から、Neovim 本体のネイティブ API である `vim.lsp.config()` / `vim.lsp.enable()` への移行が進んでいます。

| 世代 | 書き方 | 状態 |
| --- | --- | --- |
| 旧 | `require('lspconfig').lua_ls.setup({})` | 動作するが非推奨へ移行中 |
| 新（0.11） | `vim.lsp.enable("lua_ls")` | 推奨。nvim-lspconfig は設定定義の提供に専念 |

新方式では `nvim-lspconfig` はサーバーの設定値（`cmd`, `filetypes`, `root_markers` 等）を定義する役割に絞られ、実際の起動は `vim.lsp.enable()` が担います。config のカスタマイズが必要な場合は `vim.lsp.config()` で上書きできます。

## 設定ファイル

`~/.config/nvim/lua/plugins/lsp.lua`:

```lua
return {
  -- 外部ツール管理
  {
    "mason-org/mason.nvim",
    opts = {
      registries = {
        "github:mason-org/mason-registry",
        "github:Crashdummyy/mason-registry",
      },
    },
  },

  -- Mason と lspconfig の橋渡し
  {
    "mason-org/mason-lspconfig.nvim",
    dependencies = { "mason-org/mason.nvim" },
    opts = {},
  },

  -- LSP 設定定義
  {
    "neovim/nvim-lspconfig",
    config = function()
      vim.lsp.enable("lua_ls")
      vim.lsp.enable("rust_analyzer")
      vim.lsp.enable("basedpyright")
    end,
  },

  -- 補完
  {
    "saghen/blink.cmp",
    version = "*",
    opts = {
      keymap = { preset = "default" },
      appearance = { nerd_font_variant = "mono" },
      completion = {
        documentation = { auto_show = true },
      },
      sources = {
        default = { "lsp", "path", "snippets", "buffer" },
      },
    },
  },

  -- 構文解析
  {
    "nvim-treesitter/nvim-treesitter",
    build = ":TSUpdate",
    opts = {
      ensure_installed = {
        "lua",
        "vim",
        "vimdoc",
        "query",
        "markdown",
        "markdown_inline",
        "c_sharp",
        "rust",
        "python",
        "toml",
        "json",
        "yaml",
      },
      highlight = { enable = true },
      indent = { enable = true },
    },
    config = function(_, opts)
      require("nvim-treesitter.configs").setup(opts)
    end,
  },

  -- 保存時自動整形
  {
    "stevearc/conform.nvim",
    opts = {
      format_on_save = { timeout_ms = 1000, lsp_format = "fallback" },
      formatters_by_ft = {
        lua = { "stylua" },
        python = { "ruff_format" },
        rust = { "rustfmt" },
        cs = { "csharpier" },
        markdown = { "prettier" },
      },
    },
  },
}
```

## 各プラグインの役割と設計判断

### mason.nvim — 外部ツールの管理

Mason は LSP サーバー、フォーマッター、リンターなどの外部ツールを Neovim の中から管理するパッケージマネージャーです。

`registries` に2つのレジストリを指定しています。

| レジストリ | 用途 |
| --- | --- |
| `mason-org/mason-registry` | 公式レジストリ。大半のツールはここにある |
| `Crashdummyy/mason-registry` | C# の Roslyn LSP サーバーを含む追加レジストリ。roslyn.nvim で必要 |

#### Mason で管理するツールと管理しないツールの境界

Mason は便利ですが、すべてを Mason に任せるのが良いわけではありません。

| ツール | 管理方法 | 理由 |
| --- | --- | --- |
| `lua-language-server` | Mason | Lua 専用。Mason が最も手軽 |
| `stylua` | Mason | Lua フォーマッター。Mason が手軽 |
| `prettier` | Mason | 汎用フォーマッター。Mason で十分 |
| `rust-analyzer` | **rustup** | ツールチェーンのバージョンと整合を取るため |
| `rustfmt` | **rustup** | 同上 |
| `basedpyright` | **uv** | Python ツールチェーンで管理した方が環境と整合する |
| `ruff` | **uv** | 同上 |
| `csharpier` | **dotnet** | .NET ツールとして管理するのが自然 |
| `roslyn` | **Mason（追加レジストリ）** | roslyn.nvim が Mason 経由でのインストールを前提とする |

Mason でインストールするツール:

```vim
:MasonInstall lua-language-server stylua prettier
```

### mason-lspconfig.nvim — Mason と lspconfig の橋渡し

Mason でインストールした LSP サーバーの実行パスを nvim-lspconfig に自動で渡す橋渡し役です。明示的な設定は不要で、依存関係として宣言するだけで動きます。

### nvim-lspconfig — LSP 設定定義

Neovim 0.11 の新方式では、`vim.lsp.enable()` を呼ぶだけでサーバーが有効になります。サーバーの設定定義（`cmd`, `filetypes`, `root_markers` 等）は nvim-lspconfig が提供します。

今回有効化するサーバー:

| サーバー | 言語 | 説明 |
| --- | --- | --- |
| `lua_ls` | Lua | Neovim の設定ファイル自体の補完・診断に使う |
| `rust_analyzer` | Rust | rustup でインストールした rust-analyzer を使う |
| `basedpyright` | Python | uv でインストールした basedpyright を使う |

C# の Roslyn は nvim-lspconfig ではなく roslyn.nvim が管理するため、ここには書きません。詳細は [[05 言語別設定]] を参照してください。

#### サーバー設定をカスタマイズしたい場合

`vim.lsp.config()` でデフォルト設定を上書きできます。例:

```lua
vim.lsp.config("basedpyright", {
  settings = {
    basedpyright = {
      analysis = {
        typeCheckingMode = "standard",
      },
    },
  },
})
vim.lsp.enable("basedpyright")
```

### blink.cmp — 補完

blink.cmp は高速な補完プラグインです。Rust で書かれたファジーマッチャーを使うため、nvim-cmp より補完候補の表示が速いです。

設定のポイント:

| 項目 | 設定 | 意味 |
| --- | --- | --- |
| `keymap.preset` | `"default"` | `<C-n>` / `<C-p>` で選択、`<C-y>` で確定 |
| `documentation.auto_show` | `true` | 候補を選んだときにドキュメントを自動表示する |
| `sources.default` | `lsp, path, snippets, buffer` | 補完ソースの優先順。LSP → パス → スニペット → バッファ内テキスト |

> **なぜ nvim-cmp ではないか**: nvim-cmp は 9.2k stars で圧倒的な定番ですが、Lua ベースのマッチャーを使っています。blink.cmp は Rust ベースで、特に候補が多い場合のレスポンスで優位です。新規構築なら blink.cmp が合います。

### nvim-treesitter — 構文解析

treesitter は正規表現ベースではなく AST（抽象構文木）ベースで構文ハイライトとインデントを行います。

- `ensure_installed`: 使う言語のパーサーを列挙する。初回起動時に自動ダウンロードされる
- `highlight.enable = true`: treesitter ベースのハイライトを有効化
- `indent.enable = true`: treesitter ベースのインデントを有効化
- `build = ":TSUpdate"`: プラグイン更新時にパーサーも更新する

> パーサーのビルドには C コンパイラが必要です。[[01 Ubuntu Neovim 環境 OS 基盤セットアップ]] で `build-essential` をインストールしているのはこのためです。

### conform.nvim — 保存時自動整形

保存時にフォーマッターを自動実行します。

| 言語 | フォーマッター | インストール方法 |
| --- | --- | --- |
| Lua | stylua | Mason |
| Python | ruff_format | uv (`ruff` に含まれる) |
| Rust | rustfmt | rustup |
| C# | csharpier | dotnet tool |
| Markdown | prettier | Mason |

`lsp_format = "fallback"` は、指定フォーマッターがない場合に LSP のフォーマット機能を使うという意味です。

## 外部ツールのインストールまとめ

OS 基盤セットアップで入れたものと合わせて、この時点で必要な外部ツールの一覧です。

```bash
# Mason で入れるもの（Neovim 内で実行）
:MasonInstall lua-language-server stylua prettier

# uv で入れるもの（01 で実行済み）
uv tool install basedpyright
uv tool install ruff

# rustup で入れるもの（01 で実行済み）
rustup component add rust-analyzer

# dotnet で入れるもの（01 で実行済み）
dotnet tool install -g csharpier
```

## 動作確認

1. Neovim を起動して `:Lazy sync` を実行する
2. `:MasonInstall lua-language-server stylua prettier` を実行する
3. Lua ファイルを開いて補完が効くことを確認する（`vim.` と入力して候補が出る）
4. ファイルを保存して自動整形されることを確認する
5. `:checkhealth` を実行して LSP 関連でエラーがないことを確認する

## この時点の状態

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
        └── lsp.lua     ← 今回追加
```

## 次に読む文書

[[05 言語別設定]] に進み、C#、Rust、Python、Markdown の言語固有設定を追加します。
