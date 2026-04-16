# Neovim コア設定

## この文書の責務

Neovim の設定ファイル群の骨格を作ります。具体的には、プラグインマネージャー **lazy.nvim** の導入、エディタの基本オプション、キーマッピングの3つを扱います。プラグイン個別の設定は後続の文書で扱います。

## 前提

- [[01 Ubuntu Neovim 環境 OS 基盤セットアップ]] が完了していること
- Neovim 0.12.1 がインストール済みであること
- `~/.config/nvim/lua/config/` と `~/.config/nvim/lua/plugins/` ディレクトリが存在すること

## 1. init.lua — エントリポイント

Neovim は起動時に `~/.config/nvim/init.lua` を読みます。このファイルは設定の入口として、リーダーキーの定義と各モジュールの読み込みだけを担います。

`~/.config/nvim/init.lua`:

```lua
vim.g.mapleader = " "
vim.g.maplocalleader = " "

require("config.options")
require("config.keymaps")
require("config.lazy")
```

### リーダーキーについて

`mapleader` はスペースキーに設定します。Neovim のキーマッピングでは `<leader>` というプレフィックスが慣習的に使われ、スペースキーが最も押しやすい位置にあるためです。**この設定は lazy.nvim の読み込みより前に書く必要があります。** プラグイン定義内の `keys` テーブルでリーダーキーを参照する場合に、タイミングが遅れるとマッピングが効きません。

## 2. options.lua — エディタ基本オプション

エディタの見た目と基本挙動を設定します。

`~/.config/nvim/lua/config/options.lua`:

```lua
vim.opt.number = true
vim.opt.relativenumber = true
vim.opt.mouse = "a"
vim.opt.clipboard = "unnamedplus"
vim.opt.ignorecase = true
vim.opt.smartcase = true
vim.opt.termguicolors = true
vim.opt.signcolumn = "yes"
vim.opt.updatetime = 200
vim.opt.splitright = true
vim.opt.splitbelow = true
vim.opt.completeopt = { "menu", "menuone", "noselect" }
```

### 各オプションの意味

| オプション | 値 | 効果 |
| --- | --- | --- |
| `number` | `true` | 行番号を表示する |
| `relativenumber` | `true` | カーソルからの相対行番号を表示する。`5j` のような相対移動が楽になる |
| `mouse` | `"a"` | 全モードでマウス操作を有効化する |
| `clipboard` | `"unnamedplus"` | ヤンクとペーストをシステムクリップボードと共有する |
| `ignorecase` | `true` | 検索で大文字小文字を区別しない |
| `smartcase` | `true` | 検索に大文字が含まれる場合だけ大文字小文字を区別する |
| `termguicolors` | `true` | 24bit カラーを有効化する |
| `signcolumn` | `"yes"` | 左端のサイン列を常時表示し、Git diff や診断マーカーのたびにレイアウトがズレるのを防ぐ |
| `updatetime` | `200` | CursorHold イベントの発火間隔を短縮する。gitsigns 等のレスポンスが上がる |
| `splitright` | `true` | 垂直分割で新しいウィンドウを右に開く |
| `splitbelow` | `true` | 水平分割で新しいウィンドウを下に開く |
| `completeopt` | `menu,menuone,noselect` | 補完メニューの挙動。blink.cmp と組み合わせて使う |

## 3. keymaps.lua — キーマッピング

プラグインに依存するキーマッピングと、基本操作のキーマッピングをまとめます。

`~/.config/nvim/lua/config/keymaps.lua`:

```lua
local map = vim.keymap.set

-- 基本操作
map("n", "<leader>w", "<cmd>w<cr>", { desc = "Save" })
map("n", "<leader>q", "<cmd>q<cr>", { desc = "Quit" })

-- ファイル管理（oil.nvim）
map("n", "-", "<cmd>Oil<cr>", { desc = "Open parent directory" })

-- 検索（fzf-lua）
map("n", "<leader>ff", function() require("fzf-lua").files() end, { desc = "Find files" })
map("n", "<leader>fg", function() require("fzf-lua").live_grep() end, { desc = "Live grep" })
map("n", "<leader>fb", function() require("fzf-lua").buffers() end, { desc = "Buffers" })
map("n", "<leader>fr", function() require("fzf-lua").oldfiles() end, { desc = "Recent files" })
map("n", "<leader>gs", function() require("fzf-lua").git_status() end, { desc = "Git status" })

-- LSP
map("n", "<leader>gd", vim.lsp.buf.definition, { desc = "Go to definition" })
map("n", "<leader>gr", vim.lsp.buf.references, { desc = "References" })
map("n", "<leader>ca", vim.lsp.buf.code_action, { desc = "Code action" })
map("n", "<leader>rn", vim.lsp.buf.rename, { desc = "Rename symbol" })

-- Markdown
map("n", "<leader>mp", "<cmd>MarkdownPreviewToggle<cr>", { desc = "Markdown preview" })
```

### キーマッピングの設計方針

キーマッピングは以下の規則で整理しています。

- `<leader>f` 系: **Find** — ファイル検索・テキスト検索
- `<leader>g` 系: **Go / Git** — 定義ジャンプ、Git 操作
- `<leader>c` 系: **Code** — コードアクション
- `<leader>r` 系: **Rename / Recent** — リネーム、最近のファイル
- `<leader>m` 系: **Markdown** — Markdown 関連
- `-`: oil.nvim（ファイラー）。Vim の慣習的な上位ディレクトリキーを踏襲

`desc` は which-key.nvim でキー一覧を表示したときに使われます。省略しないでください。

## 4. lazy.lua — プラグインマネージャーのブートストラップ

lazy.nvim はインストールスクリプトを自分で実行する pre-bootstrap 方式を採用しています。初回起動時に自動で clone し、以降は自動更新します。

`~/.config/nvim/lua/config/lazy.lua`:

```lua
local lazypath = vim.fn.stdpath("data") .. "/lazy/lazy.nvim"
if not vim.uv.fs_stat(lazypath) then
  vim.fn.system({
    "git",
    "clone",
    "--filter=blob:none",
    "https://github.com/folke/lazy.nvim.git",
    "--branch=stable",
    lazypath,
  })
end
vim.opt.rtp:prepend(lazypath)

require("lazy").setup("plugins", {
  checker = { enabled = true },
  change_detection = { notify = false },
})
```

### 動作の流れ

1. `~/.local/share/nvim/lazy/lazy.nvim` に lazy.nvim 本体があるか確認する
2. なければ GitHub から stable ブランチを clone する
3. `runtimepath` の先頭に追加する
4. `lua/plugins/` ディレクトリ配下の全 `.lua` ファイルをプラグイン定義として読み込む

### オプションの意味

| オプション | 効果 |
| --- | --- |
| `checker.enabled = true` | プラグインの更新チェックを自動で行う |
| `change_detection.notify = false` | 設定ファイル変更時の通知を抑制する（開発中に煩わしいため） |

### lazy.nvim の基本操作

| コマンド | 効果 |
| --- | --- |
| `:Lazy` | lazy.nvim の管理画面を開く |
| `:Lazy sync` | プラグインのインストール・更新・削除を一括実行する |
| `:Lazy health` | プラグインのヘルスチェックを実行する |
| `:Lazy profile` | 起動時間のプロファイルを確認する |

## この時点の状態

ここまでで、以下の4ファイルが作成されています。

```text
~/.config/nvim/
├── init.lua
└── lua/
    └── config/
        ├── lazy.lua
        ├── options.lua
        └── keymaps.lua
```

ただし、`lua/plugins/` にはまだファイルがないため、Neovim を起動すると lazy.nvim の初期画面が表示されるだけです。プラグイン定義は次の文書で作成します。

## 次に読む文書

[[03 プラグイン設定（検索・ファイル・UX）]] に進み、fzf-lua、oil.nvim、snacks.nvim 等のプラグイン定義を追加します。
