# Ubuntu Dev Ansible

Ubuntu 24.04 向けの個人開発環境を、`zsh` と `Neovim` を中心に再現可能な形で構築する Ansible です。

この構成は次をまとめて扱います。

- 共通パッケージ: `curl`, `git`, `software-properties-common`, `ripgrep`, `fd-find`, `fzf`, `tmux`, `eza`, `zoxide`, `nodejs`, `npm`
- zsh: `zinit`, `starship`, `.zshrc`, `starship.toml`
- Neovim: `nvim` 本体、`uv`, `rustup`, `.NET SDK`, `csharpier`
- Neovim 設定: `init.lua`, `lua/config/*`, `lua/plugins/*`
- 初回ブートストラップ: `Lazy sync`, `MasonInstall`, `TSUpdateSync`

## 対応範囲

- 対象 OS: Ubuntu 24.04 LTS
- 想定アーキテクチャ: amd64
- 想定ユーザー: `sudo` 権限を持つ通常ユーザー

> Neovim の `.deb` は amd64 前提です。arm64 を使う場合は `group_vars/all.yml` の URL とアーキテクチャ変数を調整してください。

## ディレクトリ構成

```text
infra/ansible/ubuntu_dev/
├── ansible.cfg
├── group_vars/
│   └── all.yml
├── inventories/
│   └── localhost.ini
├── playbooks/
│   └── site.yml
└── roles/
    ├── common/
    │   ├── defaults/main.yml
    │   └── tasks/main.yml
    ├── zsh/
    │   ├── defaults/main.yml
    │   ├── tasks/main.yml
    │   └── templates/
    │       ├── zshrc.j2
    │       └── starship.toml.j2
    └── neovim/
        ├── defaults/main.yml
        ├── tasks/main.yml
        └── templates/
            ├── init.lua.j2
            └── lua/
                ├── config/
                └── plugins/
```

## 前提条件

制御側または対象ホスト側に Ansible が必要です。

Ubuntu 上でローカル適用する最小例:

```bash
sudo apt update
sudo apt install -y ansible
```

## 使い方

### 1. 変数を確認する

主に触るのは `group_vars/all.yml` です。

代表的な変数:

| 変数 | 意味 | 既定値 |
| --- | --- | --- |
| `ubuntu_dev_user` | 設定を適用するユーザー | 実行ユーザー |
| `ubuntu_dev_change_default_shell` | zsh をデフォルトシェルにするか | `true` |
| `ubuntu_dev_starship_version` | starship の導入バージョン | `1.24.2` |
| `ubuntu_dev_neovim_version` | Neovim の導入バージョン | `0.12.1` |
| `ubuntu_dev_apt_lock_timeout` | apt / dpkg ロック待機の上限秒数 | `600` |
| `ubuntu_dev_apt_wait_poll_interval` | apt 待機中の状態確認間隔 | `5` |
| `ubuntu_dev_neovim_bootstrap_plugins` | `Lazy` / `Mason` / `TSUpdate` を走らせるか | `true` |

### 2. inventory を確認する

ローカル適用の既定 inventory は `inventories/localhost.ini` です。

```ini
[ubuntu_dev]
localhost ansible_connection=local
```

別ホストに入れる場合は、同じ形式でホストを追加してください。

### 3. playbook を実行する

```bash
cd infra/ansible/ubuntu_dev
ansible-playbook playbooks/site.yml -K
```

`-K` は `become` 用の sudo パスワード入力です。

実行時は標準出力に次を表示します。

- 対象ホスト、ユーザー、Ubuntu バージョン、アーキテクチャ
- 導入予定の主要バージョン
- apt / dpkg ロック待機設定
- パッケージマネージャーが busy な場合の待機状況

`unattended-upgrades` などが apt ロックを掴んでいる場合は、`ubuntu_dev_apt_lock_timeout` の範囲で待機してから続行します。

### 4. 役割ごとに実行する

zsh だけ:

```bash
ansible-playbook playbooks/site.yml -K --tags zsh
```

Neovim だけ:

```bash
ansible-playbook playbooks/site.yml -K --tags neovim
```

ドライラン:

```bash
ansible-playbook playbooks/site.yml -K --check --diff
```

## 何が入るか

### zsh

- `~/.zshrc`
- `~/.config/starship.toml`
- `~/.local/share/zinit/zinit.git`
- `~/.local/bin/starship`

`.zshrc` は次を含みます。

- `compinit` のキャッシュ最適化
- `zsh-autosuggestions` の手動 rebind
- `zsh-syntax-highlighting` の長文抑制
- `fzf`, `zoxide`, `eza`, `tmux` と相性の良い alias / keybind

### Neovim

- `~/.config/nvim/init.lua`
- `~/.config/nvim/lua/config/*.lua`
- `~/.config/nvim/lua/plugins/*.lua`
- `.NET SDK 10.0`
- `lazy.nvim` によるプラグイン同期
- `mason.nvim` による `lua-language-server`, `stylua`, `prettier`, `roslyn` 導入
- `treesitter` の初期更新

## 検証コマンド

適用後は少なくとも次を確認してください。

```bash
zsh --version
starship --version
echo $SHELL
nvim --version
rg --version
fd --version
uv --version
rustc --version
dotnet --version
```

Neovim 側の確認:

```bash
nvim --headless "+checkhealth" +qa
```

## 運用上の注意

- `.zshrc` と Neovim 設定は template が正本です
- 手で直した内容は再実行時に上書きされます
- 変更は `group_vars/all.yml` か `roles/*/templates/` に寄せてください

## 関連ドキュメント

- [docs/tech_notes/linux/ubuntu/00 Ubuntu オレオレ設定ガイド.md](../../../docs/tech_notes/linux/ubuntu/00%20Ubuntu%20オレオレ設定ガイド.md)
- [docs/tech_notes/linux/zsh/00 zsh ドキュメントガイド.md](../../../docs/tech_notes/linux/zsh/00%20zsh%20ドキュメントガイド.md)
- [docs/tech_notes/linux/neovim/00 Neovim ドキュメントガイド.md](../../../docs/tech_notes/linux/neovim/00%20Neovim%20ドキュメントガイド.md)
