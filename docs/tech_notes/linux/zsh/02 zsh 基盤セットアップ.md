# zsh 基盤セットアップ

## この文書の責務

zsh 本体のインストール、デフォルトシェルの変更、プラグインマネージャー zinit の導入、プロンプト starship の導入を扱います。ここまでで zsh が使える状態にし、`.zshrc` の中身は次の文書で設定します。

## 前提

- Ubuntu 24.04 LTS
- `sudo` 権限あり
- インターネット接続あり

## 1. zsh のインストール

```bash
sudo apt update
sudo apt install -y zsh
```

インストール確認:

```bash
zsh --version
```

`zsh 5.9` 以上が表示されれば問題ありません。Ubuntu 24.04 では 5.9 が入ります。

### zsh のバージョンと機能

| バージョン | 主な追加機能 |
| --- | --- |
| 5.0 | `compinit` の高速化、新しいグロブ修飾子 |
| 5.8 | `${(z)...}` のフラグ拡張、補完の改善 |
| 5.9 | Unicode 処理の改善、補完の互換性向上 |

5.9 であればこの構成で必要な機能はすべて使えます。

## 2. デフォルトシェルの変更

現在のデフォルトシェルを確認します。

```bash
echo $SHELL
```

`/bin/bash` と表示される場合は、zsh に変更します。

```bash
chsh -s $(which zsh)
```

**この変更は次回ログイン時に反映されます。** 即座に zsh を試すには `zsh` と入力してください。

### chsh が効かない場合

Docker コンテナや一部の WSL 環境では `chsh` が使えないことがあります。その場合は `~/.bashrc` の末尾に以下を追加するフォールバックがあります。

```bash
if [ -x "$(command -v zsh)" ]; then
  exec zsh
fi
```

ただし、これは bash を起動してから zsh に切り替える方式であり、起動速度でわずかに不利です。可能な限り `chsh` を使ってください。

### 初回起動時のセットアップウィザード

zsh を初めて起動すると、設定ファイルがない場合にセットアップウィザードが表示されます。`q` を押してスキップしてください。この文書の手順で `.zshrc` を作成するため、ウィザードの設定は不要です。

## 3. 基本ディレクトリの準備

```bash
mkdir -p ~/.local/bin
```

`~/.local/bin` は後述の starship や各種ツールのインストール先として使います。

## 4. zinit のインストール

zinit は zsh のプラグインマネージャーです。oh-my-zsh と異なり、遅延ロード（lazy loading）をネイティブにサポートしており、プラグインを増やしても起動速度を維持できます。

```bash
bash -c "$(curl --fail --show-error --silent --location https://raw.githubusercontent.com/zdharma-continuum/zinit/HEAD/scripts/install.sh)"
```

インストール先は `~/.local/share/zinit` です。

### なぜ zinit か

zsh のプラグインマネージャーは複数存在します。

| マネージャー | 特徴 |
| --- | --- |
| **zinit** | 遅延ロード・Turbo Mode をネイティブサポート。最も詳細な制御が可能 |
| sheldon | Rust 製で高速。TOML で宣言的に設定。シンプル志向 |
| antidote | oh-my-zsh 系からの移行に向く。設定が簡単 |
| zplug | 機能は豊富だがメンテナンスが停滞 |
| oh-my-zsh | フレームワーク型。遅延ロード不可で重い |

zinit を選ぶ最大の理由は **Turbo Mode** です。プラグインのロードを zsh のプロンプト表示後まで遅延させることで、体感起動速度を大幅に改善できます。

```bash
# 即時ロード（遅い）
zinit light zsh-users/zsh-autosuggestions

# 遅延ロード（速い）— Turbo Mode
zinit ice wait lucid
zinit light zsh-users/zsh-autosuggestions
```

`wait` はプロンプトが表示されてからバックグラウンドでロードする指示です。`lucid` はロード完了時のメッセージを抑制します。ユーザーがコマンドを入力し始める頃にはロードが完了しているため、体感的な遅延はありません。

### zinit の構造

```text
~/.local/share/zinit/
├── bin/           ← zinit 本体
├── plugins/       ← ダウンロードされたプラグイン
├── snippets/      ← oh-my-zsh 等から取り込んだスニペット
└── completions/   ← 補完定義ファイル
```

## 5. starship のインストール

starship は Rust 製のクロスシェルプロンプトです。bash, zsh, fish, PowerShell など多くのシェルで使えます。

```bash
curl -sS https://starship.rs/install.sh | sh
```

インストール先は `/usr/local/bin/starship` です。

インストール確認:

```bash
starship --version
```

### なぜ starship か

| プロンプト | 言語 | 速度 | 設定 | クロスシェル |
| --- | --- | --- | --- | --- |
| 手書き PS1 | Shell | ◎ | × 煩雑 | × |
| powerlevel10k | Zsh | ○ | ○ | × zsh 専用 |
| **starship** | Rust | ◎ | ◎ TOML | ◎ |

powerlevel10k は zsh 専用で高機能ですが、zsh を離れた場合に知識が再利用できません。starship は Rust 製で高速かつ、どのシェルでも同じプロンプトが使えます。設定は `~/.config/starship.toml` の TOML ファイルで、git ブランチ、言語バージョン、コマンド実行時間など必要なモジュールだけを有効にできます。

### starship の初期設定

設定ファイルは `~/.config/starship.toml` です。存在しなければデフォルト設定で動作します。カスタマイズ例:

```toml
# ~/.config/starship.toml

# WSL から /mnt/c 以下を見ることが多い場合は既定値 30ms では短すぎる
scan_timeout = 2000

# プロンプト直前の空行を無効化
add_newline = false

# コマンド実行時間を表示する閾値（ミリ秒）
[cmd_duration]
min_time = 500

# Node.js のバージョン表示を無効化（不要な場合）
[nodejs]
disabled = true

# Python の仮想環境名を表示
[python]
format = 'via [${symbol}${pyenv_prefix}(${version} )(\($virtualenv\) )]($style)'
```

WSL では PowerShell から `wsl` を起動したとき、開始ディレクトリが `/mnt/c/Users/...` になることがあります。この場所は Linux ネイティブのファイルシステムより走査が遅く、starship の既定値 `scan_timeout = 30` では現在ディレクトリの判定が間に合わず、`Scanning current directory timed out` の警告が出ることがあります。そのため、Windows 側ディレクトリをよく跨ぐ運用なら、最初から `scan_timeout = 2000` 程度にしておく方が安定します。

## 6. .zshrc の雛形作成

この時点で最小限の `.zshrc` を作成します。プラグイン設定は次の文書で追加します。

```bash
cat << 'EOF' > ~/.zshrc
# ==== PATH ====
export PATH="$HOME/.local/bin:$PATH"

# ==== zinit ====
source "${HOME}/.local/share/zinit/zinit.git/zinit.zsh"

# ==== starship ====
eval "$(starship init zsh)"
EOF
```

## 7. 動作確認

新しいシェルを起動して確認します。

```bash
zsh
```

以下を確認してください。

1. starship のプロンプトが表示されること（デフォルトでは `❯` 記号）
2. Git リポジトリ内で移動するとブランチ名が表示されること
3. `zinit` コマンドが認識されること（`zinit --help`）

## 確認チェックリスト

```bash
zsh --version       # zsh 5.9+
starship --version  # starship X.Y.Z
echo $SHELL         # /usr/bin/zsh（または /bin/zsh）
```

## この時点の状態

```text
~/.zshrc                      ← 最小構成（PATH + zinit + starship）
~/.local/share/zinit/         ← zinit 本体
~/.config/starship.toml       ← starship 設定（任意）
```

## 次に読む文書

[[03 zshrc 設定とプラグイン]] に進み、プラグインの追加、補完、キーバインド、エイリアスを設定します。
