# Ubuntu オレオレ設定ガイド

## この文書の責務

Ubuntu 24.04 上で使う個人開発環境の「オレオレ設定」を、手動ドキュメントと Ansible 自動構築の両方から辿れる入口として整理します。

このガイドの対象は、単なるツール紹介ではなく、**実際に常用する設定ファイル群**と**その適用手段**です。

## 前提

- OS: Ubuntu 24.04 LTS
- 想定用途: 個人開発端末、WSL、学習用 Ubuntu 環境
- 主な対象: zsh / starship / CLI ツール群 / Neovim / Ansible 自動構築

## この構成は本当に最低限か

このガイドでいう「最低限」は、Ubuntu を素で使うための最低限ではありません。**自分の開発端末を毎回ほぼ同じ状態に再現するための最低限**です。

そのため、この文書では次の 3 層を最小単位として扱います。

- 端末作業の基盤: zsh / starship / CLI ツール群
- 編集作業の基盤: Neovim 本体とコア設定
- 再現性の基盤: Ansible による自動構築

Neovim のプラグイン群は「入れても入れなくてもよいおまけ」ではなく、検索、LSP、整形、言語別支援を担う**拡張レイヤー**です。Ubuntu 基盤と混ぜずに、コア設定とプラグイン設定を分けて把握する必要があります。

## 文書一覧

| # | 文書 | 役割 |
| --- | --- | --- |
| 1 | [[01 Ubuntu オレオレ設定一覧]] | 何を入れるか、どの設定ファイルがあるかを一覧で把握する |
| 2 | [[02 Ubuntu オレオレ設定の使い方]] | 手動適用と Ansible 適用の入口をまとめて確認する |
| 3 | [[../ansible/00 Ansible 基礎ガイド]] | Ansible の基本概念と `ubuntu_dev` の読み方を学ぶ |
| 4 | [[../zsh/00 zsh ドキュメントガイド]] | zsh 系設定の詳細を読む |
| 5 | [[../neovim/00 Neovim ドキュメントガイド]] | Neovim 系設定の詳細を読む |

## 構成全体像

```text
Ubuntu オレオレ設定
├── Core
│   ├── Ubuntu 基盤
│   │   ├── zsh          ─ シェル本体、補完、履歴、キーバインド
│   │   ├── starship     ─ プロンプト表示
│   │   └── CLI ツール群 ─ fzf / zoxide / rg / fd / eza / tmux
│   └── Neovim コア
│       ├── init.lua     ─ 設定の入口
│       ├── options.lua  ─ 基本挙動
│       ├── keymaps.lua  ─ キーマップ
│       └── lazy.lua     ─ プラグイン読込基盤
├── Plugins
│   └── Neovim プラグイン
│       ├── core.lua      ─ 検索 / ファイル / UX
│       ├── lsp.lua       ─ LSP / 補完 / 整形 / treesitter
│       ├── csharp.lua    ─ C# 固有拡張
│       └── lang-extra.lua ─ Rust / Markdown などの追加拡張
└── Automation
    └── Ansible          ─ 上記を再現可能に適用する自動構築レイヤー
```

## レイヤーごとの責務

| レイヤー | 主な対象 | 役割 |
| --- | --- | --- |
| Ubuntu Core | zsh / starship / CLI ツール群 | 端末上の基本操作を安定させる |
| Neovim Core | `init.lua` / `lua/config/*.lua` | エディタの基本動作とプラグイン読込の土台を作る |
| Neovim Plugins | `lua/plugins/*.lua` | 検索、LSP、補完、整形、言語別支援を追加する |
| Automation | `infra/ansible/ubuntu_dev/` | 端末を再現可能な形でまとめて適用する |

ここで重要なのは、**Neovim のコア設定と Neovim のプラグイン設定は別レイヤー**だという点です。`lazy.nvim` 自体はコア側に置き、実際のプラグイン定義は `lua/plugins/` 配下に分離しています。

## Neovim のコアとプラグイン

Neovim は 1 つの設定ファイルで全部を抱えず、コアとプラグインを分けて管理します。コアは「Neovim がどう起動し、どの設定群を読むか」を決め、プラグインは「何を追加でできるようにするか」を決めます。

### コア設定

| ファイル | 役割 |
| --- | --- |
| `init.lua` | リーダーキー設定と各モジュールの読み込み入口 |
| `lua/config/options.lua` | 行番号、検索、split、clipboard などの基本挙動 |
| `lua/config/keymaps.lua` | 共通キーマップ |
| `lua/config/lazy.lua` | `require("lazy").setup("plugins")` により `lua/plugins/` を読み込む |

### プラグイン設定

| ファイル | 主な中身 | 役割 |
| --- | --- | --- |
| `lua/plugins/core.lua` | which-key、snacks、fzf-lua、oil、gitsigns | 検索、ファイル操作、UI/UX |
| `lua/plugins/lsp.lua` | mason、mason-lspconfig、nvim-lspconfig、blink.cmp、treesitter、conform | LSP、補完、整形、構文解析 |
| `lua/plugins/csharp.lua` | roslyn.nvim | C# 向け拡張 |
| `lua/plugins/lang-extra.lua` | rustaceanvim、render-markdown、markdown-preview | Rust / Markdown などの追加拡張 |

`lazy.nvim` は `lua/plugins/` 配下の Lua ファイルを自動で読み込むため、プラグインは「機能単位」でファイルを分けるのが基本方針です。検索系は `core.lua`、LSP 系は `lsp.lua`、言語固有は専用ファイルという切り分けにしてあります。

## Neovim プラグインを追加する方法

この構成では、プラグイン追加は「ローカルの Neovim 設定を書き換える」だけでは終わりません。**どのレイヤーに属するかを決め、必要なら Ansible の正本にも反映する**ところまでが追加作業です。

- まず追加先を決める: 検索・ファイル操作・UX なら `lua/plugins/core.lua`、LSP・補完・整形なら `lua/plugins/lsp.lua`、言語固有なら既存の言語ファイルか新しい `lua/plugins/<domain>.lua` を使います。

- 正本を決める: その場で試すだけなら `~/.config/nvim/lua/plugins/` を直接編集して構いませんが、継続運用するなら `infra/ansible/ubuntu_dev/roles/neovim/templates/lua/plugins/` 配下の `.j2` テンプレートを更新します。

- lazy.nvim のプラグイン定義を書く: 最小形は次のようになります。

```lua
return {
  {
    "author/plugin-name.nvim",
    event = "VeryLazy",
    opts = {},
  },
}
```

- 外部依存を揃える: LSP やフォーマッター系なら `:MasonInstall`、Treesitter を増やすなら `ensure_installed` と `:TSUpdateSync`、OS パッケージが必要なら apt 側も確認します。

- 反映して確認する: Neovim で `:Lazy sync` を実行し、必要なら `:Lazy health` や `:checkhealth` で確認します。Ansible 管理下なら最後に `playbooks/site.yml` を再実行して、ローカルの試験変更と自動構築の状態がズレないようにします。

既存ファイルに収まりが悪い場合は、新しい `lua/plugins/<topic>.lua` を作って構いません。`lazy.lua` 側は `require("lazy").setup("plugins")` でディレクトリ単位に読み込むため、追加ファイルを明示登録する必要はありません。

## どこを見るべきか

- まず何が入るか見たい場合: [[01 Ubuntu オレオレ設定一覧]]
- どう適用するかだけ知りたい場合: [[02 Ubuntu オレオレ設定の使い方]]
- コア設定とプラグイン設定の境界を把握したい場合: この文書の「レイヤーごとの責務」と「Neovim のコアとプラグイン」を見る
- Neovim のプラグイン追加手順を追いたい場合: この文書の「Neovim プラグインを追加する方法」を見た上で、[[../neovim/02 Neovim コア設定]] と [[../neovim/03 プラグイン設定（検索・ファイル・UX）]] を読む
- zsh の設計意図まで読みたい場合: [[../zsh/00 zsh ドキュメントガイド]]
- Neovim のプラグイン構成まで読みたい場合: [[../neovim/00 Neovim ドキュメントガイド]]

## Ansible 資産

Ansible の実体は次にあります。

```text
infra/ansible/ubuntu_dev/
```

詳細な使い方は以下を参照してください。

```text
infra/ansible/ubuntu_dev/README.md
```
