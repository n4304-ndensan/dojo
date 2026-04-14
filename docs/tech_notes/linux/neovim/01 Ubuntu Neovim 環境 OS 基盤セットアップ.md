# Ubuntu Neovim 環境 OS 基盤セットアップ

## この文書の責務

Neovim 0.11 系の開発環境を Ubuntu 24.04 上に構築するための、OS レベルの準備手順を示します。ここで扱うのは OS パッケージ、言語別ツールチェーン（uv / rustup / dotnet）、Neovim 本体のインストールまでです。Neovim の設定ファイルには触れません。

## 前提

- Ubuntu 24.04 LTS（amd64）
- インターネット接続あり
- `sudo` 権限あり

## 1. OS パッケージのインストール

Neovim プラグインや外部ツールが依存する基本パッケージを先に入れます。

```bash
sudo apt update
sudo apt install -y \
  curl git unzip build-essential \
  ripgrep fd-find fzf \
  tmux luarocks nodejs npm
```

`fd-find` は Ubuntu ではバイナリ名が `fdfind` になるため、`fd` としてアクセスできるようシンボリックリンクを作ります。

```bash
mkdir -p ~/.local/bin
ln -sf "$(which fdfind)" ~/.local/bin/fd
```

`~/.local/bin` を PATH に通します。すでに通っている場合はスキップしてください。

```bash
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

> **シェル前提**: この文書では zsh を使用します。zsh 環境がまだない場合は [[00 zsh ドキュメントガイド]] を先に参照してください。

### 入るもの一覧

| パッケージ | 用途 |
| --- | --- |
| `curl`, `git`, `unzip` | ダウンロード・展開の基本ツール |
| `build-essential` | C コンパイラ（treesitter パーサーのビルドに必要） |
| `ripgrep` | fzf-lua の live_grep バックエンド |
| `fd-find` | fzf-lua のファイル検索バックエンド |
| `fzf` | fzf-lua が内部で使う fzf 本体 |
| `tmux` | ターミナルマルチプレクサ（任意だが推奨） |
| `luarocks` | Lua パッケージマネージャー（一部プラグインが使用） |
| `nodejs`, `npm` | markdown-preview.nvim 等のビルドに必要 |

## 2. Python ツールチェーン — uv

Python のパッケージ管理には **uv** を使います。`pip`, `venv`, `pipx` を一本化でき、仮想環境の作成・パッケージインストールが高速です。

```bash
curl -LsSf https://astral.sh/uv/install.sh | sh
source ~/.zshrc
```

インストール確認:

```bash
uv --version
```

Python 向けの開発ツールは uv で入れます。これらは後の LSP・整形設定で使います。

```bash
uv tool install basedpyright
uv tool install ruff
```

> **なぜ uv か**: `pip` + `venv` + `pipx` の組み合わせでも同じことはできますが、uv は単一バイナリでこれらすべてを代替し、速度も桁違いに速いです。2024 年以降の Python ツールチェーンとして定番になりつつあります。

## 3. Rust ツールチェーン — rustup + cargo

Rust のツールチェーンは **rustup** で管理します。`rust-analyzer` も rustup 経由でインストールできます。

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"
```

インストール確認:

```bash
rustc --version
cargo --version
```

rust-analyzer を追加します:

```bash
rustup component add rust-analyzer
```

> **なぜ rustup 経由か**: Mason でも rust-analyzer はインストールできますが、rustup で入れた方がツールチェーンのバージョンと整合が取れます。Mason 側では rust-analyzer を管理対象外にし、rustup 側に任せるのが安定します。

## 4. C# ツールチェーン — dotnet

.NET SDK は Microsoft の公式リポジトリから入れます。

```bash
sudo apt install -y dotnet-sdk-9.0
```

> Ubuntu 24.04 では Microsoft リポジトリを追加しなくても `dotnet-sdk-9.0` が利用可能です。利用できない場合は [Microsoft の公式手順](https://learn.microsoft.com/dotnet/core/install/linux-ubuntu) に従ってリポジトリを追加してください。

インストール確認:

```bash
dotnet --version
```

C# のフォーマッターは dotnet tool で入れます:

```bash
dotnet tool install -g csharpier
```

> **なぜ dotnet tool か**: CSharpier は .NET ツールとして配布されており、dotnet tool での管理が最も自然です。Mason でもインストール可能ですが、SDK バージョンとの整合性は dotnet tool の方が確実です。

## 5. Neovim 本体のインストール

Ubuntu 24.04 の `apt` 標準リポジトリにある Neovim は 0.9.5 です。0.11 系のネイティブ LSP 機能（`vim.lsp.config()` / `vim.lsp.enable()`）を使うため、公式リリースの `.deb` を直接インストールします。

```bash
cd /tmp
curl -LO https://github.com/neovim/neovim-releases/releases/download/stable/nvim-linux-x86_64.deb
sudo apt install -y ./nvim-linux-x86_64.deb
```

インストール確認:

```bash
nvim --version
```

出力に `NVIM v0.11` 以上が表示されることを確認してください。

### apt 版との競合について

既に `apt install neovim` で旧バージョンが入っている場合は、先に `sudo apt remove neovim` で削除してから `.deb` をインストールしてください。公式 `.deb` は `/usr/bin/nvim` に配置されるため、パスの競合はありません。

## 6. 設定ディレクトリの作成

Neovim の設定ファイルを配置するディレクトリ構造を作ります。

```bash
mkdir -p ~/.config/nvim/lua/config
mkdir -p ~/.config/nvim/lua/plugins
```

この時点でのディレクトリ構造:

```text
~/.config/nvim/
└── lua/
    ├── config/    ← options, keymaps, lazy.nvim ブートストラップ
    └── plugins/   ← プラグイン定義ファイル群
```

## 確認チェックリスト

以下がすべて通れば、OS 基盤のセットアップは完了です。

```bash
nvim --version        # NVIM v0.11.x
uv --version          # uv X.Y.Z
rustc --version       # rustc X.Y.Z
cargo --version       # cargo X.Y.Z
dotnet --version      # 9.0.xxx
rg --version          # ripgrep X.Y.Z
fd --version          # fd X.Y.Z
node --version        # vXX.Y.Z
basedpyright --version
ruff --version
```

## 次に読む文書

[[02 Neovim コア設定]] に進み、lazy.nvim と基本設定を構成します。
