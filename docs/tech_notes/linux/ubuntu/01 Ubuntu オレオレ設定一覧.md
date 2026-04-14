# Ubuntu オレオレ設定一覧

## この文書の責務

Ubuntu 端末に入れる設定と設定ファイルを、**何のためのものか**がすぐ分かる形で一覧化します。

使い方や適用手順は別文書の [[02 Ubuntu オレオレ設定の使い方]] に分離します。

## 1. シェルとプロンプト

| 名前 | 種別 | 配置先 | どのようなものか |
| --- | --- | --- | --- |
| zsh | シェル | `/usr/bin/zsh` | bash から置き換える対話シェル。補完、履歴、プラグイン運用の中心 |
| `.zshrc` | ユーザー設定 | `~/.zshrc` | PATH、補完、履歴、キーバインド、alias、fzf、zoxide、zinit 読み込みを定義する本体設定 |
| zinit | プラグインマネージャー | `~/.local/share/zinit/zinit.git` | zsh プラグインを宣言的に管理し、遅延ロードで起動速度を保つ |
| `starship.toml` | プロンプト設定 | `~/.config/starship.toml` | Git ブランチ、実行時間、Python 環境などを見やすく表示するプロンプト設定 |

## 2. CLI ツール群

| 名前 | 種別 | 主な配置先 | どのようなものか |
| --- | --- | --- | --- |
| fzf | 検索ツール | `/usr/bin/fzf` | 履歴、ファイル、ディレクトリ、一覧選択を高速化するファジーファインダー |
| zoxide | 移動ツール | `/usr/bin/zoxide` | 訪問履歴を学習して `z foo` でディレクトリ移動できる |
| ripgrep (`rg`) | テキスト検索 | `/usr/bin/rg` | `grep -r` の代替。コードベース横断検索を高速化する |
| fd | ファイル検索 | `~/.local/bin/fd` | Ubuntu の `fdfind` へ張る互換リンク。`find` の代替として使う |
| eza | 一覧表示 | `/usr/bin/eza` | `ls` の代替。色、アイコン、Git 状態付きで見やすい |
| tmux | 端末多重化 | `/usr/bin/tmux` | セッション維持、ペイン分割、長時間作業向け |

## 3. Neovim 本体

| 名前 | 種別 | 配置先 | どのようなものか |
| --- | --- | --- | --- |
| Neovim | エディタ本体 | `/usr/bin/nvim` | 0.11 系を前提にした CLI エディタ |
| `init.lua` | エントリポイント | `~/.config/nvim/init.lua` | リーダーキー定義と各設定モジュールの読み込みを行う |
| `lua/config/options.lua` | 基本設定 | `~/.config/nvim/lua/config/options.lua` | 行番号、検索、split、clipboard などの基本挙動を定義する |
| `lua/config/keymaps.lua` | キーマップ | `~/.config/nvim/lua/config/keymaps.lua` | 保存、検索、LSP、Markdown などのショートカットを定義する |
| `lua/config/lazy.lua` | プラグイン起動設定 | `~/.config/nvim/lua/config/lazy.lua` | lazy.nvim のブートストラップとプラグイン読み込みを定義する |

## 4. Neovim プラグイン設定

| 名前 | 種別 | 配置先 | どのようなものか |
| --- | --- | --- | --- |
| `lua/plugins/core.lua` | 検索・UX 設定 | `~/.config/nvim/lua/plugins/core.lua` | which-key、snacks、fzf-lua、oil、gitsigns をまとめて定義する |
| `lua/plugins/lsp.lua` | LSP・補完・整形 | `~/.config/nvim/lua/plugins/lsp.lua` | mason、lspconfig、blink.cmp、treesitter、conform を定義する |
| `lua/plugins/csharp.lua` | C# 設定 | `~/.config/nvim/lua/plugins/csharp.lua` | roslyn.nvim を使った C# 向け設定 |
| `lua/plugins/lang-extra.lua` | Rust / Markdown 設定 | `~/.config/nvim/lua/plugins/lang-extra.lua` | rustaceanvim、render-markdown、markdown-preview をまとめる |

## 5. 言語ツールチェーン

| 名前 | 種別 | 主な配置先 | どのようなものか |
| --- | --- | --- | --- |
| uv | Python ツールチェーン | `~/.local/bin/uv` | Python のパッケージ・ツール管理を一本化する |
| basedpyright | Python LSP | `~/.local/bin/basedpyright` | Python の型チェックと補完に使う |
| ruff | Python 整形/診断 | `~/.local/bin/ruff` | Python のフォーマットと lint を担当する |
| rustup / cargo | Rust ツールチェーン | `~/.cargo/bin` | Rust 本体、rust-analyzer、rustfmt を管理する |
| dotnet SDK | C# ツールチェーン | `/usr/bin/dotnet` | Roslyn 系ツールや C# 開発基盤を支える |
| csharpier | C# フォーマッター | `~/.dotnet/tools/csharpier` | C# の保存時整形に使う |

## 6. Ansible 自動構築

| 名前 | 種別 | 配置先 | どのようなものか |
| --- | --- | --- | --- |
| `ansible.cfg` | Ansible 設定 | `infra/ansible/ubuntu_dev/ansible.cfg` | inventory や roles_path などの実行設定 |
| inventory | 対象ホスト定義 | `infra/ansible/ubuntu_dev/inventories/localhost.ini` | localhost や Ubuntu ホストを定義する |
| `group_vars/all.yml` | 変数設定 | `infra/ansible/ubuntu_dev/group_vars/all.yml` | 対象ユーザー、バージョン、挙動フラグをまとめる |
| `playbooks/site.yml` | 実行入口 | `infra/ansible/ubuntu_dev/playbooks/site.yml` | common / zsh / neovim をまとめて適用する playbook |
| `roles/common` | 共通処理 | `infra/ansible/ubuntu_dev/roles/common/` | apt パッケージ、基本ディレクトリ、fd 互換リンクを扱う |
| `roles/zsh` | zsh 構築 | `infra/ansible/ubuntu_dev/roles/zsh/` | zinit、starship、`.zshrc`、デフォルトシェル変更を扱う |
| `roles/neovim` | Neovim 構築 | `infra/ansible/ubuntu_dev/roles/neovim/` | Neovim 本体、uv、rustup、dotnet、設定ファイル、プラグイン初期化を扱う |

## 7. この一覧の見方

- 「何が入るか」を把握したいときはこの一覧を見る
- 「どう適用するか」は [[02 Ubuntu オレオレ設定の使い方]] を見る
- zsh / Neovim の設計理由まで追いたい場合は、それぞれの詳細ドキュメントを見る
